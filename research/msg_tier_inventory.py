"""Independent MSG HTML -> live_handlers output inventory (stdlib, read-only).

Usage: python3 research/msg_tier_inventory.py SOURCE_HTML LIVE_HANDLER_LOG
The live example truncates labels to 48 chars and omits variants; this checks
every source label/threshold/amount, not just counts. Variant/Store behavior is
covered by Rust parser -> record -> TempDB tests, not by this script.
"""

import hashlib
import json
import re
import sys
from collections import Counter
from decimal import Decimal
from html.parser import HTMLParser
from pathlib import Path


class Node:
    def __init__(self, tag="", attrs=(), parent=None):
        self.tag, self.attrs, self.parent = tag, dict(attrs), parent
        self.children, self.parts = [], []

    def ancestors(self):
        node = self
        while node:
            yield node
            node = node.parent

    def has_class(self, name):
        return name in self.attrs.get("class", "").split()

    def hidden(self):
        return any(n.has_class("w-condition-invisible") for n in self.ancestors())

    def text(self):
        if self.hidden():
            return ""
        return "".join(p if isinstance(p, str) else p.text() for p in self.parts)

    def descendants(self):
        for child in self.children:
            yield child
            yield from child.descendants()


class Document(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.current = Node()
        self.nodes = []

    def handle_starttag(self, tag, attrs):
        node = Node(tag, attrs, self.current)
        self.current.children.append(node)
        self.current.parts.append(node)
        self.nodes.append(node)
        if tag not in {"br", "img", "input", "link", "meta", "hr", "source", "wbr", "area", "embed", "param", "col", "base"}:
            self.current = node

    def handle_endtag(self, tag):
        node = self.current
        while node.parent and node.tag != tag:
            node = node.parent
        if node.parent:
            self.current = node.parent

    def handle_data(self, data):
        self.current.parts.append(data)


def inventory(source, output):
    document = Document()
    document.feed(source.decode())
    cards = [n for n in document.nodes if n.tag == "a" and n.has_class("product4_text-link")
             and any(a.tag == "section" and a.has_class("section_product4") for a in n.ancestors())]
    gaps = {"Alu Cu Kühler", "Stahl- und Gussspäne", "Messer versilbert", "Silberbesteck 800",
            "Versilbertes Besteck gestempelt 60/40/20", "Versilbertes Besteck gestempelt 80/90/100"}
    expected, missing_catalog, products = Counter(), [], []
    for card in cards:
        if card.hidden():
            continue
        title = next(" ".join(n.text().split()) for n in card.descendants() if n.has_class("text-weight-semibold"))
        quotes, zeros, hidden = [], 0, 0
        for row in card.descendants():
            if not row.has_class("pricedisplay"):
                continue
            if row.hidden():
                hidden += 1
                continue
            cells = [" ".join(c.text().split()) for c in row.children if c.tag == "div" and not c.hidden()]
            assert len(cells) == 3 and cells[2] == "€", (title, cells)
            if not cells[1]:
                continue
            amount = Decimal(cells[1].replace(",", "."))
            assert amount >= 0
            if amount == 0:
                zeros += 1
                continue
            quotes.append({"threshold": cells[0], "eur_per_kg": str(amount)})
            label = f"{title} ({cells[0]})"
            if title in gaps:
                missing_catalog.append({"title": title, "label": label, "price": str(amount)})
            else:
                expected[(label[:48], amount)] += 1
        products.append({"title": title, "quotes": quotes, "zero_rows": zeros, "hidden_rows": hidden})
    actual = Counter()
    for line in output.splitlines():
        assert "FAILED:" not in line, line
        match = re.search(r"\s([0-9.]+)\s+EUR/kg\s+conf=.*\[(.*)\]$", line)
        if match:
            actual[(match[2], Decimal(match[1]))] += 1
    assert expected == actual, {"missing": list((expected - actual).items()), "unexpected": list((actual - expected).items())}
    gap_lines = [line for line in output.splitlines() if "SKIP: Kataloglücke:" in line]
    for gap in missing_catalog:
        assert any(gap["title"] + " (" + gap["label"].rsplit(" (", 1)[1][:-1] in line for line in gap_lines), gap
    assert len(gap_lines) == len(missing_catalog)
    return {"source_sha256": hashlib.sha256(source).hexdigest(), "cards": len(products),
            "positive_distribution": dict(Counter(len(p["quotes"]) for p in products)),
            "positive_source_quotes": sum(len(p["quotes"]) for p in products),
            "matched_handler_quotes": sum(actual.values()), "catalog_gaps": missing_catalog,
            "products": products}


if __name__ == "__main__":
    print(json.dumps(inventory(Path(sys.argv[1]).read_bytes(), Path(sys.argv[2]).read_text()), ensure_ascii=False, indent=2))

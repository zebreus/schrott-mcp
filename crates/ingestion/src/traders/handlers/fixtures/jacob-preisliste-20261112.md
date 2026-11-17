# Metallhandel Jens Jacob operator price-list PDF fixture

Fetched 17.11.2026 from the homepage's "PREISLISTE" link:
https://www.metallhandel-jacob.de/media/files/preisliste-standard_2026-11-12-01.pdf

SHA-256: `4bf865daf6d3ad0d4734acaa086a8699a808f9167ee979277c2a8a8ede3bf921`.

One page, an "Aushang Preisliste" dated **12.11.2026**, two columns of roughly
100 rows (material designation, unit, "Preis (€/Einheit)"). The footer names
the general *Einkaufbedingungen* (minimum one kilogram, special prices on
agreement) and the reverse-charge procedure per §13b Abs. 2 Nr. 7 UStG. A
second file with the same bytes is linked as
`media/files/preisliste-standard_2026-11-12.pdf`; a third PDF
(`konditionen-containerdienstleitstungen_neu.pdf`) holds the container-service
fees (50,00 EUR per trip) and explains neither sign.

**Why the prices are not recorded:** the metal rows carry **negative** values
(e.g. "Kupfer Draht blank (Milberry)" -9,20 EUR/kg) while the service rows
carry positive ones ("Lohnverwiegung" 10,00 EUR/St, "Schutt/Beton*" 250,00
EUR/t). Neither the price list nor the conditions state what the sign means -
a deduction from the daily quotation, a surcharge, or a book-entry convention.
Without that explanation a negative number is not an absolute purchase price,
so the handler records the **accepted sorts** (from the same list) and leaves
the prices out until the meaning is documented.

Regression tests extract the actual PDF offline.

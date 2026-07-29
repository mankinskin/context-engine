# Structured Ticket Entities

Extend the ticket system and ticket entity beyond the existing task packages that address agents overwriting ticket descriptions while adding updates such as review results. The system should make such accidental full-description replacement harder or impossible.

- Give ticket entities a stronger, deeper structure by storing related content in multiple files within the ticket entity directory instead of placing everything in one file. Treat these files as attachments.
- Store ticket reviews and acceptance-criteria reviews as independent documents, and integrate them through the ticket manifest. This keeps the ticket's core objective description protected and avoids the need to edit it for updates.
- Consider freezing tickets or ticket descriptions at an appropriate point so planned information cannot later be edited away.
- Split commonly occurring ticket content into separate files, including exact requirements, design, examples, and other relevant material. This should provide stronger structure and support better planning units.
- Allow ticket-system read commands to combine these files and provide aggregated views of a ticket. Keep this interface configurable so agents can request only the information they need rather than loading every ticket detail into their context.
- Make each ticket a rich, deeply grounded, self-contained mini-plan. It should contain the information relevant to planning and processing the ticket while referring to external entities or context through typed references.
- Provide a comprehensive, finely calibrated or configurable query system that lets agents extract specific ticket information and edit tickets without allowing fully planned or already started tickets to be changed in ways that lose planned steps or delete context about past work or accepted decisions.

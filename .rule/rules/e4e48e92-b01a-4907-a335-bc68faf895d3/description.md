**Security**
- [ ] User-controlled strings inserted into the DOM use text-node APIs (e.g.
      `set_text_content`) — not `set_inner_html` — to prevent XSS.
- [ ] URLs derived from external data are validated before fetch/navigation.
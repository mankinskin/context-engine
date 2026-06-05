# Stdin from another process
echo -e "create --title 'Setup CI' --type tracker-improvement\nclose <UUID>" \
  | ticket batch --toon
```
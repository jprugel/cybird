# Cybird
The intent behind this project is to create an api for both developers of mods and games.

# Priorities
- Ensure that the API is easy to implement by game developers and allows for them to add any number of data oriented features directly into the game.

# To Do:
- [x] Implement simple api for developers to implement a single feature that is moddable.\
- [x] Expand to multi feature api
- [ ] Allow game developers to create a simple mod collection manager with a single command using just.
- [ ] Expand just commands to focus on extension.

# Guide
1. Create an enum that contains Tuple types of any extendable type.
2. Implement Registrable on the inner type.
3. Implement Context on the containing type.

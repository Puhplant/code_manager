## Guidance

1. Use the melt component library for building the UI where possible. Ensure that we use it as svelte 5
2. If more complex components are required then search the code_manager_web code base first in the components folder for existing components that could be easily adapted before creating new components
3. we are using Svelte5 that means we need to use runes for managing state ($state) and $effect for side effects etc, and do not use the on:click syntax as it is depreciated we use onclick instead. Check when done that there are no depreciated warnings and resolve them
4. avoid leaving comments in the code
5. avoid making any configuration changes to the app
6. avoid making demos and readmes
7. Styles are done use tailwind 4. Use the theme described in the main.css file. use tailwind and not the css variables for colors
8. using the request.ts file for matching api request rather than the fetch api directly

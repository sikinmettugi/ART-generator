# Album Review Template Generator

A Rust webapp for generating blog post template designated for album reviews

Current blog is: [locustspectre.name](https://locustspectre.name)



## What Does the App Exactly Do

- Gets search keyword
- Look up for keyword in iTunes Store
  - Spotify and|or other streaming service coming soon
- When the appropriate content is chosen, generates a md template for content review post

## TODO
- [x] POC
- [ ] Make a running webapp backend with `actix-web`
- [ ] Make a running webapp frontend (highly likely it'll be [svelte](https://svelte.dev/) )
- [ ] Setup CI/CD with either Azure DevOps or Github Actions
  - [ ] … with deploying on free AWS instance or other free services
- [ ] Refactor code 
  - [ ] Revise directory structure if necessary
- [ ] Make/implement a trait for search (i.e. where to search)

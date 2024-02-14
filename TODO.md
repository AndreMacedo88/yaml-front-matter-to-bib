# Main issues to solve
- make citation styles more modular by removing them from front_matter_types.rs and putting them each into their own folder
  - how to import them back to front_matter_types.rs or main.rs?

# Functionalities to implement
- Deal with missing citation fields
  - Instead of erroring, try to generate the citation with only the fields that were present
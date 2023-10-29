module.exports = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [2, "always", ["chore", "feat", "fix", "refactor"]],
    "scope-enum": [2, "always", ["brainfuck", "common"]],
  },
};

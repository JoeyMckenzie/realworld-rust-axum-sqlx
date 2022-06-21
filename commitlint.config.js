module.exports = {
    extends: ["@commitlint/config-conventional"],
    parserPreset: {
        parserOpts: {
            headerPattern: /^(\w*)\((\w*)\):\s(.*)$/,
            headerCorrespondence: ["type", "scope", "subject"]
        }
    },
    rules: {
        "scope-empty": [1, "never"],
    },
    helpUrl:
        "https://github.com/conventional-changelog/commitlint/#what-is-commitlint",
};
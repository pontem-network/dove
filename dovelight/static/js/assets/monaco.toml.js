window.addEventListener("load", function() {
    // Register a new language
    monaco.languages.register({ id: 'toml' });

    // @todo 
    console.log(monaco.languages);

    // Register a tokens provider for the language
    monaco.languages.setMonarchTokensProvider('toml', {
        tokenizer: {
            root: [
                [/^\[[^\]]+\]/, "metatag"],
                [/^\w+/, "key"],
            ]
        }
    });
    // Define a new theme that contains only rules that match this language 
    monaco.editor.defineTheme('tomlTheme', {
        base: 'vs',
        inherit: true,
        rules: [
            { token: 'metatag', foreground: '808080' },
            { token: 'value2', foreground: '123456' },
        ]
    });
});
# snipple

snippet managing support tooling for vim-snipple


## Usage

```
% snipple
Usage: snipple [COMMAND]

Commands:
  get            Get a specific snippet
  list-snippets  List all available snippets
  version        Print version and exit
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


```
% snipple list
go/main.snippet
go/actions-worflow.snippet
go/config-package.snippet
go/Makefile.snippet
go/test.snippet
go/read-yaml-file.snippet
notes/ref-template.snippet
ruby/test.snippet
```

```
% snipple get go/test.snippet

package test

import (
        "github.com/stretchr/testify/assert"
        "testing"
)

func TestSomething(t *testing.T) {
        assert := assert.New(t)
        tests := map[string]struct {
                input string
                sep   string
                want  []string
        }{
                "simple": {input: "a/b/c", sep: "/", want: []string{"a", "b",
"c"}},
        }

        for name, tc := range tests {
                t.Run(name, func(t *testing.T) {
                        // execute test logic here
                        assert.Equal(nil, err)
                })
        }
}
```



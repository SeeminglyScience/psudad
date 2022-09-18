# `psudad`

`psudad` aka "PS Utility Decode and Deflate" is a small console app that decodes base64 encoded gzip
strings and writes the content to stdout.

The primary (and likely exclusive) use case for this is sending large pre-built strings to render in
`fzf`'s preview pane without hitting the `cmd` character limit for lines. If the preview you want to
generate would come from PowerShell, and `fzf` is being invoked from PowerShell, this lets you avoid
the high start up cost of re-entry.

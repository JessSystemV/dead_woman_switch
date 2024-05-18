# Dead Woman's Switch

A dead man's switch made to e-*suspend* your linux PC when a drive is unplugged.

Inspired by TailsOS, idea to make this by JessSystemV, written by TudbuT, modified again by JessSystemV to accomodate her setup,
Which closes the LUKS drive upon suspend.


```
$ sudo dws /dev/sdb & disown
```

## Disclaimer

I do not take any liability for any damages done by this software. Use this
only on PCs you don't mind losing data on.


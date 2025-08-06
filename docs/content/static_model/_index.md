+++
title = "Track Plans"
type = "chapter"
weight = 1
+++

## Abstract Track Layout

```ebnf
abstract track layout  = track line, { track line } ;

track line             = element [ ";" ] | (  element, "-", track line ) 
                            | ( element, track line ) ;

element                = track | point | ds point | ss point | derailer
                            | crossing | non blocking crossing ;

track                 = "[", id, [ ",", id ] "]" ;
point                 = "W[", id, ",", id, "]" ;
ds point              = "DKW[", id, ",", id, ",", id, "]" ;
ss point              = "EKW[", id, ",", id, "]" ;
derailer              = "Gs[", id, "]" ;
crossing              = "K[", id, "]" ;
non blocking crossing = "T[", id, "]" ;

id = ? ASCII alphanumeric, '-' and '_'  ?
```
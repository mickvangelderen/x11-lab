
## Prevent use after free

```
window = display.create_window()

// Shouldn't be allowed while we hold a reference to a window.
drop(display)

window.set_title("bla");
```

or

```
screen = display.default_screen()
window = display.create_window()

// Shouldn't be allowed while we hold a reference to a window.
display.delete_all_windows()

// Should be allowed
display.change_default_screen(screen + 1)

window.set_title("bla");
```

which means we need granular control. We can't allow some of display to be mutated while we borrow another part without actually splitting the display into multiple parts.

## Prevent use in wrong context

```
d1 = display();
d2 = display();

let screen = d1.default_screen();

// Shouldn't be allowed since screen represents a resource from d1, not d2. 
let window = d2.root_window(screen)
```

I see two ways to tackle this issue statically:
    1. screen should own a copy of d1, downside: potentially larger structs than
       necessary. a macro could generate a struct that captures multiple
       resource associations.
    2. generate a display implementation for very display.

# What is this?

This projects was created to give rust a try. The code might not make a lot of sense. If you have hints on what to do
differently, then please leave an issue!

There is a practical purpose for this code. There is the cmd tool called [taskwarrior](https://taskwarrior.org/). This
tool is supposed to work with the [i3 Window Manager](https://i3wm.org/) and a plugin for it called 
[i3blocks](https://github.com/vivien/i3blocks).

It will give you the current status of timewarrior in the bar. This will look as follows:

When there is an active time tracking (tags: i3block test timewarrior; duration: almost 25 min):

![Section of a status bar reading'i3block test timewarrior [0:24:48]'](/images/timewarrior-status-bar-active.png)

When no time tracking is active:

![Section of a status bar reading 'No time tracking active'](/images/timewarrior-status-bar-inactive.png)


# Setup
Since I use [regolith](https://regolith-linux.org/) I am not sure about the the exact set up for pure i3blocks -
specifically the i3blocks folder and how to refresh the view.

1. Build the `src/main.rs` using `rustc`. Put the binary into the a path variable path (i.e. `/usr/local/bin`).
2. Put the `03_timewarrior-status` file into your i3blocks folder.
3. Refresh the view (posslibly try [mod] + [shift] + r)
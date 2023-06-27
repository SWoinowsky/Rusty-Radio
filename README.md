# Rusty-Radio
Terminal-Based Spotify Radio application for linux devices where a web player is too intensive.


# Why?
When I was in highschool, I was absolutely enthralled by small computer systems like the Raspberry Pi, Latte Panda, and NextThingCo's (Rest in peace) CHIP. There was, at one point, a terminal app that connected to Pandora and I loved having such a lightweight way to interact with the service. Since I found that a missing spot in my life, I decided I'd try my hand at making my own. 


# What's to see here?
I'll be documenting what I do, what I find, and how I take steps to learning to do this project as a notebook for myself and a roadmap for people interested in making their own.


# Learning TUI
Here I'll be following [This Tutorial](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/) by a Mario Zupan. The summarized goal is to figure out how to use TUI the TUI framework with Crossterm running everything behind-the-scenes.


## Creating Something to Work With
The first step we're going through is setting up the Rust Application base. This application will serve as my reference when I move onto integrating other things. We'll start by using Cargo to create a basic application and adding the necessary dependencies.


## New Data Structures
* We create a JSON file that holds some information like a db and we'll define a struct for our example. 

* We're also adding an Error enumerable just for the sake of managing any internal problems.

* An Event enum is also necessary to handle inputs. Reminds me a bit of programming in PICO-8.

* We've got a MenuItem enum too, suspiciously like PICO-8. I'm looking forward to this.


## Rendering Setup
The first step according to the tutorial is to set up our terminal for NonCanonical / Raw mode, which apparently means it won't have to wait for the enter key to be pressed to process an input. Then we make something called a 'Multiproducer, Single Consumer' channel. I'm thinking this will be the relationship between what we see and what we click. It's also becoming increasingly clear that things have changed between the writing of this tutorial and my attempt to replicate it. Libraries have changed some. 

Good thing the rust crates have nice documentation.

Anyways, we're also responsible for setting up the CrosstermBackend object and setting up a panic and clear if anything goes wrong with the application.

## Our First Widget
THIS IS THE PART WHERE IT GETS FAMILIAR!

So the first instruction is to set up a visual draw loop, LiKe In PiCo-8 as though I haven't mentioned it enough. I'll give that program all the free advertising I can. This is the setup for a header of only size 3, a footer of only size 3, and a main content section that grows to fit your screen, but is never smaller than 2 lines in height.

After that we have the definition of our footer, where we have a fake copyright and dictate the colors and aesthetics of it.

## Put it on my Tab
So here we have a goal of setting up tabs to help with visual navigation of the application. We added a vector called menu_titles that holds the names for the tabs, and setting the active default item to the home tab.

Then we create a menu object after our draw loop. From what I can gather, the logic is to create a list of span objects from the menu_titles vector. The first one created (the 'home' span) will be styled differently from the other ones. Then we create a Tabs object from the list of spans we made, with the styling and everything specified. Then we call the tabs to be rendered as the first section of the display. This ends up being our "header."

## The Meat and Potatoes

### Setting Up the Input Checks
So now I've got in some things that were shown to be very important for handling input. We've got a check for rx.recv(). This is the receiving end of the channel we created earlier, and is basically mapping all of the controls / inputs for our app. I've also added a render_home() function that renders the desired display for the home menu option.

### Progress so far
How cute!
![progress pic](https://i.imgur.com/FMIi2vr.png) 


use dioxus::prelude::*;

use crate::{
    compressors::image_default,
    views::art_chapters::shared::{
        ChapterPlaque, MatyPlaque, FIGURE_LEFT, FIGURE_RIGHT, FIGURE_SHARED, LINK, PICTURE,
    },
};

#[component]
pub fn Ch1() -> Element {
    rsx! {
        ChapterPlaque { title: "Maty's early work", date: "c.2016-2018",
            "My artistic journey begins during my time in
                middle-school around 2016. That time in academia
                was not much of a time sink for me and in addition to my
                incredibly lacking social life left me with a lot of free time on my hands.
                This track of my artistic journey is marked by amateurish, almost manic outbursts
                of creativity. Raw, unorganized, brutal expression
                made manifest in multiple mediums. This early work was however plagued by
                unwillingness to commit - keeping up the veneer of frivolity to evade critique.
                You cant critique that which is not taken seriously. Looking back a big part of my motivation
                to create was to make a name for myself amongst my peers. I was the weird guy who made this strange art.
                But i would have never admitted to this of course."
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/first anim c.2016.jpg"),
            }
            MatyPlaque {
                title: "Prank gone wrong",
                art_date: "c. 2016",
                art_method: "Triangles on screen (Blender)",
                div { class: "flex flex-col gap-2",
                    p {
                        "A freeze frame from the very first animation I made in blender. The woman, prankster and the gun were downloaded from the internet
                and used to tell the story of a failed prank attempt, where the prankster screams `Allāhu Akbar` and throws a fake bomb at the woman. 
                The incident ends with the woman shooting the man point blank after a short monologue criticizing prank culture. The story reflects 
                internet trends of the day, alluding to islamic terrorism
                and prank videos popular on youtube at that time."
                    }
                    a {
                        class: LINK,
                        href: "https://www.youtube.com/watch?v=RwSFGAvbwxA",
                        target: "_blank",
                        "You can watch the full animation here!"
                    }
                }

            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/christmas c.2016.png", image_default()),
            }
            MatyPlaque {
                title: "Christmas card",
                art_date: "c. 2016",
                art_method: "Triangles on screen (Blender)",
                "In this piece we see for the first time a reoccurring character known as Alicja. She was my fictional
                minecraft girl friend. But Maty, I hear you ask, aren't you a faggot? Very observant of you! Alicja was on one hand
                an ironic self degrading joke about being a looser - with my peers being the intended audience. On the other hand
                it was a way to dissuade others from thinking im a fag. How can i be a fag if i have a minecraft girlfriend? Looking back, 
                this was a really dumb thought process. We can also see a horrifying abomination of a santa to the right of the image. This style of
                art was heavily inspired by other shit-core animations that can be found on youtube at that time."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/bunkier c.2016.png", image_default()),
            }
            MatyPlaque {
                title: "Bunker",
                art_date: "c. 2016",
                art_method: "Triangles on screen (Blender)",
                "In 2016 and 2017 i was quite prolific in short animation creation. I am unable to share them, as they disclose too much personally
                identifiable information, however here is just one still from one of such animations. It depicts a dilapidated bunker with some sleeping
                pods. This artistic direction stems from my fascination with post-apocalyptic media such as the Metro, STALKER and Fallout game series. Earlier
                examples of influential media include Wall-e and Minecraft Czas series."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/rocket c.2016.png", image_default()),
            }
            MatyPlaque {
                title: "Escape pods",
                art_date: "c. 2016",
                art_method: "Triangles on screen (Blender)",
                "Here we can see another still-frame from the undisclosed animation. We can see the technical limitations
                in lighting and texturing of early blender versions.
                I would have straight up KILLED to get the modern versions of blender back then. 
                Its quite impressive I was able to do all this regardless."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/laurenty c2016.png", image_default()),
            }
            MatyPlaque {
                title: "Laurenty 1984",
                art_date: "c. 2016",
                art_method: "Triangles on screen (Blender)",
                "Yet another still-frame, this time from an unfinished animated short. Laurenty was a notorious priest antagonist present in many of my
                animations. This once again alludes to the catholic church pedophile controversies of the time. Poland had the epstein files before it was cool. Don't worry, I was not one of such victims."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/station gamma not 3d c.2017.png", image_default()),
            }
            MatyPlaque {
                title: "Station Gamma",
                art_date: "c. 2017",
                art_method: "Pencil sketch on paper, digital color",
                "Before any 3d work i always did some amateurish drawing. As stated previously, primary and middle
                school were very easy for me. You would typically find me doodling some grotesque images in my scrapbook. This one 
                isn't really grotesque, but it looks nice. The coloring was done in GIMP, line work with pencil on paper."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/dom2 c.2017.png", image_default()),
            }
            MatyPlaque {
                title: "Residence",
                art_date: "c. 2017",
                art_method: "Triangles on screen (Blender)",
                "Still-frame from the last of the short animations in a trilogy dedicated to the life of Matyniov and Alice.
                We can see the characters residing in a lavish room. A table stretches along most of it. 
                Both characters sit on opposite sides, alienated from one another, foreshadowing things to come..."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/banquet c.2017.jpg", image_default()),
            }
            MatyPlaque {
                title: "Banquet",
                art_date: "c. 2017",
                art_method: "Triangles on screen (Blender)",
                "Here we can see a still from one of my (arguably) best made animations.
                We can see two of the main characters: Dathan and Aksinya. 
                Two of them dine aboard an interstellar cruise.
                During the course of the short they fall into a one-sided love, 
                only to be abruptly interrupted by forces beyond their control..."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/space combat c.2017.jpg", image_default()),
            }
            MatyPlaque {
                title: "Space combat scene",
                art_date: "c. 2017",
                art_method: "Triangles on screen (Blender)",
                "The `forces beyond their control` alluded to in the former piece is a
                 fleet of evil space pirates. Dathan cynically gives up on his love after this event."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/minecraft c.2017.png", image_default()),
            }
            MatyPlaque {
                title: "Minecraft venus",
                art_date: "c. 2017",
                art_method: "Digital (GIMP)",
                "Around this time I also decided to get into 2D digital art. Building on my `reputation` as a freak with a minecraft girlfriend,
                I decided to create a whole series of them on DeviantArt. Multiple pieces were made, some better than others. This one in particular
                is one of the best ones. Let me remind you, I shrouded all this in pure irony."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/alien c.2018.jpg", image_default()),
            }
            MatyPlaque {
                title: "Weightless",
                art_date: "c. 2018",
                art_method: "Triangles on screen (Blender)",
                "In this piece we can see for the first time the Alien - the last main character starring in the Alice trilogy.
                Lore wise, Alice and Alien are both Aliens, who came to earth to gather data and survey humanity. 
                Matyniov had no idea about this. Looking back, the character I related to the most at that time was the Alien, not Matyniov.
                This depersonalization is emphasized by the use of TTS for Matyniov, and my real voice with filters for the Alien and Alice. 
                Lastly lets take a deeper look at the environment in this piece. As a space nerd I've always been a huge fan of saturn. 
                The clean white environment was inspired by `Alien: Prometheus` as well as `2001: A Space Odyssey`"
            }
        }
    }
}

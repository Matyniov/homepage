use dioxus::prelude::*;

use crate::views::art_chapters::shared::{
    ChapterPlaque, MatyPlaque, FIGURE_LEFT, FIGURE_RIGHT, FIGURE_SHARED, LINK, PICTURE,
};

#[component]
pub fn Ch2() -> Element {
    rsx! {
        ChapterPlaque { title: "Period of stagnation", date: "c.2018-2024",
            "Academically speaking middle school has gone by very smoothly, earning me a spot in one of the top highschools in Poland.
                This did however mean that I had to focus more of my efforts on actually studying. On top of that ive lost the spark that fueled me in middleschool.
                Your guess is as good as mine as to what caused it. In general terms I would describe this period of my life as full of uncertainty and self doubt.
                For the first time in my life I was put in an environment where i am not top of the class. This time I was about average. 
                In either case during this time I got to create some environmental pieces in Blender as well as my first furry models around the time of the COVID-19 pandemic"
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/minecraft 2 c.2019.png"),
            }
            MatyPlaque {
                title: "Minecraft venus",
                art_date: "c. 2019",
                art_method: "Digital (GIMP)",
                "Around 2019 my activity in the `Minecraft Girlfriend` space dies down. This piece is one of the last ones
                I made. This work is a parody of `The Treachery of Images` by René Magritte. The inscription at the bottom reads 
                `This is not a perfect wife`."
            }
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/furasy/V1 c.2020.png"),
            }
            MatyPlaque {
                title: "V1",
                art_date: "c. 2020",
                art_method: "Triangles on screen (Blender)",
                "Up until this point i've only been a passive consumer of furry art, never the creator.
                I've never been good at drawing, neither did I have a drawing tablet. This in my mind
                left me with one practical option: 3D. To the right you can see my first attempt at creating
                my fursona in 3D. I must admit, looking at it now its not even that bad. Goofy, amateurish, messy 
                but not an abhorrent abomination spitting in the face of god and all beauty. White wolf with red eyes. 
                How original? Less original than you think. My inspiration was a 
                character called `Dr. K` from the video game called `Changed`."
            }
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/furasy/V2 c.2021.png"),
            }
            MatyPlaque {
                title: "V2",
                art_date: "c. 2021",
                art_method: "Triangles on screen (Blender)",
                "In this iteration my model gained clothes(!). Additionally, the eyebrows were marked with the letter `M` -
                 it stands for Maty if you didn't figure it out. This aspect of my fursona remains to this day! 
                 I also moved away from the low-poly aesthetic. Smooth regions looked nicer to the eye. 
                 Is this an improvement compared to V1? Thats up to the spectator to decide."
            }
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/gamma c.2021.png"),
            }
            MatyPlaque {
                title: "Station Gamma - 3D",
                art_date: "c. 2021",
                art_method: "Triangles on screen (Blender)",
                "I enjoyed my drawing of station Gamma so much, that i decided to make it in 3D.
                The imagery and style is very clearly inspired by post-apocalyptic media as mentioned before. Its
                amazing what a few decals can do to a simple 3d model to make it pop."
            }
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/steve c.2021.png"),
            }
            MatyPlaque {
                title: "Steven",
                art_date: "c. 2021",
                art_method: "Triangles on screen (Blender)",
                "I honestly have no idea why I made this. It came to me in a dream. The spirit of art flowed through me.
                I was but a vessel that delivered this... creation. I was a god and the canvas - my universe. Mine. Craft."
            }
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/station c.2022.png"),
            }
            MatyPlaque {
                title: "Gdynia główna",
                art_date: "c. 2022",
                art_method: "Triangles on screen (Blender)",
                p {
                    "Around this time I decided to make a few mods for one of my favorite games - `Workers and Resources: Soviet Republic`.
                One of the assets I made was based on the central railway station in my hometown: Gdynia. This transportation hub is 
                close to my heart, as its the primary link between my home and my university/work. You could see it as a portal between
                adolescence and adulthood. The platform my styx, the train my charon."
                }
                a {
                    href: "https://steamcommunity.com/sharedfiles/filedetails/?id=2897591726",
                    target: "_blank",
                    class: LINK,
                    "Here you can download this mod and use it in your game!"
                }
            }
        }

        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/zbiornik c.2023.png"),
            }
            MatyPlaque {
                title: "Storage tank",
                art_date: "c. 2023",
                art_method: "Triangles on screen (Blender)",
                "Unreleased and unfinished mod that would have added a giant storage tank for chemicals. What I did create in the process however
                was this pretty rendering, that heavily uses billboarding for the background art."
            }
        }
    }
}

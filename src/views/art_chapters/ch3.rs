use crate::{
    compressors::image_default,
    views::art_chapters::shared::{
        ChapterPlaque, MatyPlaque, FIGURE_LEFT, FIGURE_RIGHT, FIGURE_SHARED, LINK, PICTURE,
    },
};
use dioxus::prelude::*;

#[component]
pub fn Ch3() -> Element {
    rsx! {
        ChapterPlaque { title: "Modern furry art", date: "c.2024-Now",
            "Now well into university I've once again found some time to
             screw around with blender after the extended stagnation. I have focused my efforts on once again
                attempting to create a furry avatar for myself.
                 This is about the time when AI \"art\" becomes more and more prevalent in the cultural zeitgeist. 
                 I consider my work a form of miniature personal revolt against that erosion of artistic value. 
                 As a reward for my struggles, I get cool renders of my fursona."
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/furasy/V3 c.2024.png", image_default()),
            }
            MatyPlaque {
                title: "V3",
                art_date: "c. 2024",
                art_method: "Triangles on screen (Blender)",
                "Third attempt at creating my fursona. White wolf with red eyes: the electric boogaloo. Moving
                away completely from hand made low-poly geometry I decided to use the sculpting tools of blender.
                This did unfortunately mean i had to add another step to my pipeline: the dreaded retopology. In the end, V3 
                came out pretty nice."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/furasy/V4 c.2025.png", image_default()),
            }
            MatyPlaque {
                title: "V4",
                art_date: "c. 2025",
                art_method: "Triangles on screen (Blender)",
                "For the fourth attempt I radically reimagined the colorscheme of my fursona.
                Instead of dull and depressing Gray-On-Gray-On-Gray with a sudden spike of red in the eyes,
                ive decided upon gentle beiges and grounded oranges and browns. The form accented by a
                geometric splash of red - in the cybernetics - and green - in the eyes. Additionally,
                 2 golden rings were added to the left ear - fancy! V4 also once again gained clothes! 
                 Quite generic, but better than nothing. In the background of this piece you can also see
                 some painterly strokes of earthy orange, pale blue and green. The inspiration for that
                 kind of background came from my enjoyment of the `Disco Elysium` video game."
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/furasy/V5 c.2025.png", image_default()),
            }
            MatyPlaque {
                title: "V5",
                art_date: "c. 2025",
                art_method: "Triangles on screen (Blender)",
                p {
                    "V5 is the final form of my fursona. On halloween I decided to add these neon antlers to him,
                but they stayed ever since because they look cool - this will become a trend. Additionally
                new clothes were made - in a cyberpunk-like style with red and black accents."
                }
                p {
                    "I don't smoke, I believe smoking is a bad vice, a menace on humanity and should be strongly discouraged.
                 BUT, god is it so fucking cool looking... To not outright have my sona be smoking (and give people the wrong
                 idea about me) I gave him the second best thing, a choco-waffle-tube."
                }

            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/most c.2026.png", image_default()),
            }
            MatyPlaque {
                title: "A bridge too far",
                art_date: "c. 2026",
                art_method: "Triangles on screen (Blender)",
                p {
                    "In this piece we can see the main accessory of Maty - a metal bat.
                    Once again, because it looks fucking cool. 
                    Canonically its used to bash fash skulls. If we zoom back again, 
                    we can see a dilapidated railway bridge. I have modeled it around 
                    2024, but I've never properly finished."
                }
                a {
                    href: "https://maps.app.goo.gl/Pju9qfN5RcyTJQAL8",
                    target: "_blank",
                    class: LINK,
                    "Here is a google maps link to the bridge that inspired me! I found it while cycling around Warsaw."
                }
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_LEFT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/works/le gun c.2026.png", image_default()),
            }
            MatyPlaque {
                title: "Whispell promotional material",
                art_date: "c. 2026",
                art_method: "Triangles on screen (Blender)",
                p {

                    "I've made this unwritten rule, that my fursona shouldn't have a gun.
                Why? Because its lazy! Guns are so blatantly cool, it would almost 
                distract from the character itself! In this case however the main
                subject is the gun, or rather, `WHISPELL 100K EM-PULSE DRONE JAMMER OF PANDEMONIUM UNITED`. This was a study of hard surface modelling."
                }
                p {
                    "For legal reasons (maybe) i need to inform you that no, i am not in fact in any way associated with żabka.
                    Ive chosen żabka, in order to depict the banality of war in this hypothetical(?) cyberpunk world. For those
                    unaware, żabkas are basically polish 711 shops that are EVERYWHERE."
                }
            }
        }
        figure { class: "{FIGURE_SHARED} {FIGURE_RIGHT}",
            img {
                class: PICTURE,
                src: asset!("/assets/imgs/furasy/V6 c.2026.png", image_default()),
            }
            MatyPlaque {
                title: "V6",
                art_date: "c. 2026",
                art_method: "Triangles on screen (Blender)",
                "Latest and greatest iteration on the furry model. In this case its not even
                my fursona! Daring, i know. This time it's a cool, serious, nonchalant, depressive and a little black-coded guy - with an actual cigarette! 
                Very woke. Next time i need to create an asian wheelchair-bound shark transwoman, because straight white man is FUCKING BORING (srs)."
            }
        }
    }
}

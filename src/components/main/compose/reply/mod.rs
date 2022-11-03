use dioxus::prelude::*;
use warp::crypto::DID;

use crate::Account;
use crate::utils;
use crate::components::ui_kit::profile_picture::PFP;

#[derive(Props, PartialEq)]
pub struct Props {
    message: String,
    is_remote: bool,
    account: Account,
    sender: DID,
}

#[allow(non_snake_case)]
pub fn Reply(cx: Scope<Props>) -> Element {
    let class = if cx.props.is_remote {
        "remote"
    } else {
        "local"
    };

    let profile_picture = utils::get_pfp_from_did(cx.props.sender.clone(), &cx.props.account.clone());
    let profile_picture2 = profile_picture.clone();

    let box_right = "🭽";
    let box_left = "🭾";

    #[cfg(target_os = "macos")]
    let box_left = "⎤";

    #[cfg(target_os = "windows")]
    let box_left = "⎤";

    #[cfg(target_os = "macos")]
    let box_right = "⎡";

    #[cfg(target_os = "windows")]
    let box_right = "⎡";


    cx.render({
        rsx! {
            div {
                class: "reply {class}",
                (cx.props.is_remote).then(|| rsx! {
                    p {
                        class: "box-drawing left",
                        "{box_right}"
                    }
                }),
                (!cx.props.is_remote).then(|| rsx! {
                    if profile_picture.is_empty() {
                        rsx! (
                            div {
                                class: "pfp"
                            }  
                        )   
                    } else {
                        rsx!(PFP {
                            src: profile_picture,
                            size: crate::components::ui_kit::profile_picture::Size::Normal
                        })
                    }
                }),
                p {
                    "{cx.props.message}",
                },
                (cx.props.is_remote).then(|| rsx! {
                    if profile_picture2.clone().is_empty() {
                        rsx! (
                            div {
                                class: "pfp"
                            }  
                        )   
                        } else {
                            rsx!(PFP {
                                src: profile_picture2,
                                size: crate::components::ui_kit::profile_picture::Size::Small
                            })
                        }
                }),
                (!cx.props.is_remote).then(|| rsx! {
                    span {
                        class: "box-drawing",
                        "{box_left}"
                    }
                })
            }
        }
    })
}

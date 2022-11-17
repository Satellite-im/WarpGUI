use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use embeds::LinkEmbed;
use linkify::LinkFinder;
use pulldown_cmark::{html, Options, Parser};

use warp::{crypto::DID, raygun::Message, raygun::TypingIndicator};

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        profile_picture::PFP,
        textarea::TextArea,
    },
    utils::{
        self,
        get_meta::{get_meta, SiteMeta},
    },
    Account, LANGUAGE,
};

pub mod embeds;

#[derive(Props)]
pub struct Props<'a> {
    message: Message,
    account: Account,
    sender: DID,
    remote: bool,
    first: bool,
    middle: bool,
    last: bool,
    on_reply: EventHandler<'a, String>,
    on_typing_reply: EventHandler<'a, TypingIndicator>,
    children: Vec<Element<'a>>,
}

#[allow(non_snake_case)]
pub fn Msg<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    log::debug!("rendering compose/Msg");
    let finder = LinkFinder::new();
    let content = cx.props.message.value();
    let joined_a = content.join("\n");
    let joined_b = joined_a.clone();
    let has_links = finder.links(&joined_b).next().is_some();

    // Parses links and grabs data like the title, favicon and description
    let fetch_meta = use_future(&cx, &joined_a, |content| async move {
        if has_links {
            let s = content.as_str();

            let links: Vec<_> = finder.links(s).collect();

            let first_link = match links.first() {
                Some(l) => l.as_str(),
                None => "",
            };
            get_meta(first_link).await
        } else {
            Ok(SiteMeta::default())
        }
    });

    let meta = match fetch_meta.value() {
        Some(Ok(val)) => val.clone(),
        Some(Err(_)) => SiteMeta::default(),
        None => SiteMeta::default(),
    };

    let popout = use_state(&cx, || false);
    // text has been lifted from the child components into Msg so that
    // a button press can be used to clear it.
    let text = use_state(&cx, String::new);
    let value = cx.props.message.clone().value().join("\n");

    let timestamp = cx.props.message.clone().date();
    let ht = utils::display_msg_time(timestamp);
    let remote = match cx.props.remote {
        true => "remote",
        false => "local",
    };
    let l = use_atom_ref(&cx, LANGUAGE).read();

    let first = match cx.props.first {
        true => "first",
        false => "",
    };

    let middle = match cx.props.middle {
        true => "middle",
        false => "",
    };

    let last = match cx.props.last {
        true => "last",
        false => "",
    };

    let hover = use_state(&cx, || false);

    let hover_class = match hover.get() {
        true => "animate_animated animate__pulse",
        false => "not-hovered",
    };

    let slide_class = match cx.props.remote {
        true => "message-wrap animate__animated animate__pulse animate__slideInLeft",
        false => "message-wrap animate__animated animate__pulse animate__slideInRight",
    };

    let profile_picture =
        utils::get_pfp_from_did(cx.props.sender.clone(), &cx.props.account.clone());
    let profile_picture2 = profile_picture.clone();
    let profile_picture3 = profile_picture.clone();

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&value, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(value.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    let (output1, output2, output3) = (
        html_output.clone(),
        html_output.clone(),
        html_output.clone(),
    );

    cx.render(rsx! (
        div {
            class: "wrapper {remote}",
            (popout).then(|| rsx!(
                div {
                    class: "popout-mask {remote}",
                    onclick: move |_| {
                        popout.set(false);
                    },
                    div {
                        class: "close",
                        IconButton {
                            icon: Shape::X,
                            on_pressed: move |_| {
                                popout.set(false);
                            }
                        },
                    },
                    div {
                        class: "message-wrap {slide_class}",
                        div {
                            class: "user-message",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            PFP {
                                src: profile_picture,
                                size: crate::components::ui_kit::profile_picture::Size::Normal
                            },
                            div {
                                class: "value popout {first} {middle} {last}",
                                div {
                                    dangerous_inner_html: "{output1}"
                                },
                            },
                        }
                        div {
                            class: "controls reply-container",
                            onclick: move |e| {
                                e.cancel_bubble();
                            },
                            IconButton {
                                icon: Shape::EmojiHappy,
                                on_pressed: move |_| {}
                            },
                            TextArea {
                                placeholder: l.send_a_reply.to_string(),
                                on_trigger_typing: move |e| {
                                    cx.props.on_typing_reply.call(TypingIndicator::Typing);
                                },
                                on_submit: move |e| {
                                    cx.props.on_reply.call(e);

                                    popout.set(false);
                                },
                                on_trigger_typing:  |_| {},
                                text: text.clone(),
                            },
                            IconButton {
                                icon: Shape::ArrowRight,
                                state: icon_button::State::Secondary,
                                on_pressed: move |_| {
                                    cx.props.on_reply.call(text.clone().to_string());
                                    popout.set(false);
                                }
                            }
                        }
                    }
                }
            )),
            div {
                class: "message {remote} {hover_class}",
                if cx.props.remote {
                    rsx! (
                        if cx.props.last {
                            rsx!(PFP {
                                src: profile_picture2,
                                size: crate::components::ui_kit::profile_picture::Size::Normal
                            })
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                        div { // todo: don't duplicate this
                            class: "value {first} {middle} {last}",
                            onclick: |_| {
                                popout.set(true);
                            },
                            onmouseover: |_| {
                                hover.set(true);
                            },
                            onmouseout: |_| {
                                hover.set(false);
                            },
                            div {
                                dangerous_inner_html: "{output2}",
                                has_links.then(|| rsx!{
                                    LinkEmbed {
                                        meta: meta
                                    }
                                }),
                            }
                        }
                    )
                } else {
                    rsx!(
                        div {
                            class: "value {first} {middle} {last}",
                            onclick: |_| {
                                popout.set(true);
                            },
                            onmouseover: |_| {
                                hover.set(true);
                            },
                            onmouseout: |_| {
                                hover.set(false);
                            },
                            div {
                                dangerous_inner_html: "{output3}",
                                has_links.then(|| rsx!{
                                    LinkEmbed {
                                        meta: meta
                                    }
                                }),
                            }
                        },
                        if cx.props.last {
                            rsx!(PFP {
                                src: profile_picture3,
                                size: crate::components::ui_kit::profile_picture::Size::Normal
                            })
                        } else {
                            rsx!( div { class: "pfp-void" } )
                        },
                    )
                }
                cx.props.last.then(|| rsx!(
                    div {
                        class: "timestamp",
                        "{ht}"
                    }
                ))
            }
        }
    ))
}

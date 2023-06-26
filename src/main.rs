use leptos::*;

#[derive(Copy, Clone)]
struct Note {
    value: (ReadSignal<i32>, WriteSignal<i32>),
    max: (ReadSignal<i32>, WriteSignal<i32>),
    ratio: (ReadSignal<i32>, WriteSignal<i32>),
}

fn s2n(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
#[component]
fn NoteItem(cx: Scope, id: i32, note: Note, set_notes: WriteSignal<Vec<(i32, Note)>> ) -> impl IntoView {
    let (value, set_value) = note.value;
    let (max, set_max) = note.max;
    let (ratio, set_ratio) = note.ratio;

    view! { cx,
        <li class="note-item">
        <form >
            <div class="form-control">
                <label for="note">"Note"</label>
                <input name="note" type="number" min="0" prop:max=max
                    on:input=move |ev| {
                       set_value(s2n(&event_target_value(&ev)));
                    }
                    prop:value=value
                />
            </div>
            <div class="form-control">
                <label for="note_max">"Max"</label>
                <input name="note" type="number" 
                    prop:min=value
                    on:input=move |ev| {
                       set_max(s2n(&event_target_value(&ev)));
                    }
                    prop:value=max
                />
            </div>
            <div class="form-control">
                <label for="note_ratio">"Ratio"</label>
                <input name="note" type="number" 
                    on:input=move |ev| {
                       set_ratio(s2n(&event_target_value(&ev)));
                    }
                    min="1"
                    prop:value=ratio
                />
            </div>
        </form>

        <button on:click= move |_| {
            set_notes.update(move |notes| {
                notes.retain(|(i, _)| *i != id)
            })
        } >"X"</button>
        </li>
    }
}

#[component]
fn NoteList(
    cx: Scope,
) -> impl IntoView {
 
    let mut next_note_id = 0;

    let (notes, set_notes) = create_signal(cx, vec![]);
    let add_note = move |_| {
        let note = Note {
            value: create_signal(cx, 0),
            max: create_signal(cx, 15),
            ratio: create_signal(cx, 1),
        };
        set_notes.update(move |notes| {
            notes.push((next_note_id, note))
        });
        next_note_id += 1;
    };
    let (total, set_total)= create_signal(cx, 0.00);
    
    let get_total = move |_| { 
        let (tt, tr) = notes().iter().map(|&(_, note)| {
            let (value, _) = note.value;
            let (max, _) = note.max;
            let (ratio, _) = note.ratio;
            let v = value() as f32;
            let m = max() as f32;
            let r = ratio() as f32;
            let t = (v / m) * 100.00 * r;
            return (t, r);
        }).reduce(|(at, ar), (bt, br)| (at + bt, ar + br)).unwrap(); 
        
        set_total(tt / tr);
    }; 
  
    view! { cx,
        <div>
            <ul class="note-list">
                <For
                    each=notes
                    key=|note| note.0
                    view=move |cx, (id, note)| {
                        view! { cx,
                            <NoteItem id=id note=note set_notes=set_notes/>
                        }
                    }
                />
            </ul>
            <footer class="total">
                <button on:click=add_note>
                    "Add Counter"
                </button>
                "Total:" {total}
                <button on:click=get_total>
                    "Get Total"
                </button>
            </footer>
        </div>
    }
}


#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <div class="app">
        <header>
            <h1>"Note Balance"</h1>
        </header>
        <main>
            <NoteList/>
        </main>
    </div>
}}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

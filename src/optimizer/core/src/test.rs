#![allow(unused_must_use)]

use super::*;
use serde_json::to_string_pretty;

macro_rules! test_input {
    ($input: expr) => {
        let input = $input;
        let res = transform_modules(TransformModulesOptions {
            root_dir: input.root_dir,
            input: vec![TransformModuleInput {
                code: input.code.clone(),
                path: input.filename,
            }],
            source_maps: true,
            minify: input.minify,
            transpile: input.transpile,
            explicity_extensions: input.explicity_extensions,
            entry_strategy: input.entry_strategy,
        });
        if input.snapshot {
            match &res {
                Ok(v) => {
                    let input = input.code.to_string();
                    let mut output = format!("==INPUT==\n\n{}", input);

                    for module in &v.modules {
                        let is_entry = if module.is_entry { "(ENTRY POINT)" } else { "" };
                        output += format!(
                            "\n============================= {} {}==\n\n{}",
                            module.path, is_entry, module.code
                        )
                        .as_str();
                        if let Some(hook) = &module.hook {
                            let hook = to_string_pretty(&hook).unwrap();
                            output += &format!("\n/*\n{}\n*/", hook);
                        }
                        // let map = if let Some(map) = s.map { map } else { "".to_string() };
                        // output += format!("\n== MAP ==\n{}", map).as_str();
                    }
                    output += format!("\n== DIAGNOSTICS ==\n\n{:?}", v.diagnostics).as_str();
                    insta::assert_display_snapshot!(output);
                }
                Err(err) => {
                    insta::assert_display_snapshot!(err);
                }
            }
        }
        drop(res)
    };
}

#[test]
fn example_1() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent, onRender } from '@builder.io/qwik';

const Header = qComponent($(() => {
  console.log("mount");
  onRender($(() => {
    return (
      <div onClick={$((ctx) => console.log(ctx))}/>
    );
  }));
}));
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_2() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
export const Header = qComponent$(() => {
  console.log("mount");
  onRender$(() => {
    return (
      <div onClick={$((ctx) => console.log(ctx))}/>
    );
  });
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_3() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
export const App = () => {
    const Header = qComponent$(() => {
        console.log("mount");
        onRende(() => {
            return (
                <div onClick={$((ctx) => console.log(ctx))}/>
            );
        });
    });
    return Header;
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_4() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
export function App() {
    const Header = qComponent$(() => {
        console.log("mount");
        return onRender$(() => {
            return (
                <div onClick={$((ctx) => console.log(ctx))}/>
            );
        });
    });
    return Header;
}
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_5() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
export const Header = qComponent$(() => {
    onRender$(() => {
        return (
            <>
                <div onClick={(ctx) => console.log("1")}/>
                <div onClick={$((ctx) => console.log("2"))}/>
            </>
        );
    })
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_6() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$ } from '@builder.io/qwik';
export const sym1 = $((ctx) => console.log("1"));
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_7() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const Header = qComponent$(() => {
    console.log("mount");
    onRender$(() => {
      return (
        <div onClick={$((ctx) => console.log(ctx))}/>
      );
    });
  });

const App = qComponent$(() => {
    onRender$(() => {
        return (
            <Header/>
        );
    })
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_8() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const Header = qComponent$(() => {
    onRender$((hola) => {
        const hola = this;
        const {something, styff} = hola;
        const hello = hola.nothere.stuff[global];
        return (
        <Header/>
        );
    });
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_9() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$ } from '@builder.io/qwik';
const Header = $((decl1, {decl2}, [decl3]) => {
    const {decl4, key: decl5} = this;
    let [decl6, ...decl7] = stuff;
    const decl8 = 1, decl9;
    function decl10(decl11, {decl12}, [decl13]) {}
    class decl14 {
        method(decl15, {decl16}, [decl17]) {}
    }
    try{}catch(decl18){}
    try{}catch({decl19}){}
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_10() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
import { $, qComponent$ } from '@builder.io/qwik';
const Header = $((decl1, {decl2}, [decl3]) => {

    const hola = ident1.no;
    ident2;
    const a = ident1 + ident3;
    const b = ident1 + ident3;
    ident4(ident5, [ident6], {ident7}, {key: ident8});
    class Some {
        prop = ident9;
        method() {
            return ident10;
        }
    }

    return (
        <div onClick={(ident11) => ident11 + ident12} required={false}/>
    )
});
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_11() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
import {foo, bar as bbar} from "../state";
import * as dep2 from "dep2";
import dep3 from "dep3/something";

export const Header = qComponent$(() => {
    onRender$(() => {
        return (
            <Header onClick={$((ev) => dep3(ev))}>
                {dep2.stuff()}{bbar()}
            </Header>
        );
    });
});

export const App = qComponent$(() => {
    onRender$(() => {
        return (
            <Header>{foo()}</Header>
        );
    })
});
"#
        .to_string(),
        entry_strategy: EntryStrategy::Single,
        ..TestInput::default()
    });
}

#[test]
fn example_12() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
export const Header = qComponent$(() => {
    onRender$(() => console.log("hello sym2"), "sym2")
});
"#
        .to_string(),
        entry_strategy: EntryStrategy::Single,
        ..TestInput::default()
    });
}

#[test]
fn example_13() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
export const Header = qComponent$(() => {
    onRender$(() => console.log("hello sym2"), "2sym")
});
"#
        .to_string(),
        entry_strategy: EntryStrategy::Single,
        ..TestInput::default()
    });
}

#[test]
fn example_functional_component() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$, createStore } from '@builder.io/qwik';
const Header = qComponent$(() => {
    const thing = createStore();
    const {foo, bar} = foo();

    onRender$(() => {
        return (
            <div>{thing}</div>
        );
    });
});
"#
        .to_string(),
        minify: MinifyMode::None,
        ..TestInput::default()
    });
}

#[test]
fn example_functional_component_2() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$, createStore } from '@builder.io/qwik';
export const useCounter = () => {
    return createStore({count: 0});
}

export const STEP = 1;

export const App = qComponent$((props) => {
    const state = useCounter();
    const thing = createStore({thing: 0});
    const STEP_2 = 2;

    return onRender$(() => {
        const count2 = state.count * 2;
        return (
            <div on$:click={() => state.count+=count2 }>
                <span>{state.count}</span>
                {buttons.map(btn => (
                    <button
                        on$:click={() => state.count += btn.offset + thing + STEP + STEP_2 + props.step}
                    >
                        {btn.name}
                    </button>
                ))}

            </div>

        )
    });
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_functional_component_capture_props() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$, createStore } from '@builder.io/qwik';

export const App = qComponent$(({count, rest: [I2, {I3, v1: [I4], I5=v2, ...I6}, I7=v3, ...I8]}) => {
    const state = createStore({count: 0});
    const {rest: [C2, {C3, v1: [C4], C5=v2, ...C6}, C7=v3, ...C8]} = foo();
    return onRender$(() => {
        return (
            <div on$:click={() => state.count += count + total }>
                {I2}{I3}{I4}{I5}{I6}{I7}{I8}
                {C2}{C3}{C4}{C5}{C6}{C7}{C8}
                {v1}{v2}{v3}
            </div>
        )
    });
})
"#
.to_string(),
transpile: true,
..TestInput::default()
});
}

#[test]
fn example_multi_capture() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const Foo = qComponent$(({foo}) => {
    const arg0 = 20;
    return onRender$(() => {
        const fn = ({aaa}) => aaa;
        return (
            <div>
              {foo}{fn()}{arg0}
            </div>
        )
    });
})

export const Bar = qComponent$(({bar}) => {
    return onRender$(() => {
        return (
            <div>
              {bar}
            </div>
        )
    });
})
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_with_tagname() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const Foo = qComponent$("my-foo", () => {
    return onRender$(() => {
        return (
            <div>
            </div>
        )
    });
})
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_with_style() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$, withStyle$ } from '@builder.io/qwik';

export const Foo = qComponent$("my-foo", () => {
    withStyle$('.class {}');
    return onRender$(() => {
        return (
            <div class="class"/>
        )
    });
})
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_lightweight_functional() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const Foo = qComponent$("my-foo", ({color}) => {
    return onRender$(() => {
        return (
            <div>
                <Button {...props} />
                <ButtonArrow {...props} />
            </div>
        )
    });
})

export function Button({text, color}) {
    return (
        <button color={color} on$:click={()=>console.log(text, color)}>{text}</button>
    );
}

export const ButtonArrow = ({text, color}) => {
    return (
        <button color={color} on$:click={()=>console.log(text, color)}>{text}</button>
    );
}
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn example_invalid_references() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

const I1 = 12;
const [I2, {I3, v1: [I4], I5=v2, ...I6}, I7=v3, ...I8] = obj;
function I9() {}
class I10 {}

export const App = qComponent$(({count}) => {
    console.log(I1, I2, I3, I4, I5, I6, I7, I8, I9);
    console.log(itsok, v1, v2, v3, obj);
    return onRender$(() => {
        return (
            <I10></I10>
        )
    });
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_invalid_hook_expr1() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$, withStyle$ } from '@builder.io/qwik';
import css1 from './global.css';
import css2 from './style.css';

export const App = qComponent$(() => {
    const style = `${css1}${css2}`;
    withStyle$(style);
    const render = () => {
        return (
            <div></div>
        )
    };
    const styles =
    return onRender$(render);
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_capture_imports() {
    test_input!(TestInput {
        code: r#"
import { qComponent$, withStyle$ } from '@builder.io/qwik';
import css1 from './global.css';
import css2 from './style.css';
import css3 from './style.css';

export const App = qComponent$(() => {
    withStyle$(`${css1}${css2}`);
    withStyle$(css3);
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_capturing_fn_class() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const App = qComponent$(() => {
    function hola() {
      console.log('hola');
    }
    class Thing {}
    class Other {}

    return onRender$(() => {
      hola();
      new Thing();
      return (
          <div></div>
      )
    });
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_renamed_exports() {
    test_input!(TestInput {
        code: r#"
import { qComponent$ as Component, onRender$ as onRender, createStore } from '@builder.io/qwik';

export const App = Component((props) => {
    const state = createStore({thing: 0});

    return onRender(() => (
        <div>{state.thing}</div>
    ));
});
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_exports() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const [a, {b, v1: [c], d=v2, ...e}, f=v3, ...g] = obj;

const exp1 = 1;
const internal = 2;
export {exp1, internal as expr2};

export function foo() { }
export class bar {}

export default function DefaultFn() {}

export const Header = qComponent$(() => {
    onRender$(() => (
        <Footer>
            <div>{a}{b}{c}{d}{e}{f}{exp1}{internal}{foo}{bar}{DefaultFn}</div>
            <div>{v1}{v2}{v3}{obj}</div>
        </Footer>
    ))
});

export const Footer = qComponent$();
"#
        .to_string(),
        ..TestInput::default()
    });
}

#[test]
fn issue_117() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
export const cache = patternCache[cacheKey] || (patternCache[cacheKey]={});
"#
        .to_string(),
        entry_strategy: EntryStrategy::Single,
        ..TestInput::default()
    });
}

#[test]
fn issue_118() {
    test_input!(TestInput {
        filename: "project/test.tsx".to_string(),
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';
import thing from 'lib';
import * as all from 'lib';
import {s as se} from 'lib';

export const Header = qComponent$(() => {
    onRender$(() => <Footer>{thing}{all()}{se()}</Footer>)
});

export const Footer = qComponent$();
"#
        .to_string(),
        entry_strategy: EntryStrategy::Single,
        minify: MinifyMode::Minify,
        transpile: true,

        ..TestInput::default()
    });
}

#[test]
fn example_jsx() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$, h, Fragment } from '@builder.io/qwik';

export const Lightweight = (props) => {
    return (
        <div>
            <>
                <div/>
                <button {...props}/>
            </>
        </div>
    )
};

export const Foo = qComponent$("my-foo", (props) => {
    return onRender$(() => {
        return (
            <div>
                <>
                    <div class="class"/>
                    <div class="class"></div>
                    <div class="class">12</div>
                </>
                <div class="class">
                    <Lightweight {...props}/>
                </div>
                <div class="class">
                    <div/>
                    <div/>
                    <div/>
                </div>
                <div class="class">
                    {children}
                </div>
            </div>
        )
    });
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_jsx_listeners() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender$ } from '@builder.io/qwik';

export const Foo = qComponent$("my-foo", () => {

    return onRender$(() => {
        const handler = $(() => console.log('reused'));
        return (
            <div
                on$:click={()=>console.log('on$:click')}
                onDocument$:scroll={()=>console.log('onDocument:scroll')}
                onWindow$:scroll={()=>console.log('onWindow:scroll')}

                on:keyup={handler}
                onDocument:keyup={handler}
                onWindow:keyup={handler}
            />
        )
    });
})
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_qwik_conflict() {
    test_input!(TestInput {
        code: r#"
import { $, qComponent$, onRender, onRender$ } from '@builder.io/qwik';

const qComponent = () => console.log('not this');
qComponent();
export const Foo = qComponent$("my-foo", () => {
    const qwik = 12;
    console.log(qwik);
    return onRender$(() => {
        return (
            <div/>
        )
    });
})

export const Root = qComponent$("my-foo", () => {
    return onRender($(() => {
        return (
            <div/>
        )
    }));
}))
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_custom_inlined_functions() {
    test_input!(TestInput {
        code: r#"
import { qComponent$, onRender$, createStore, wrap, useEffect } from '@builder.io/qwik';

export const useMemo = (qrt) => {
    useEffect(qrt);
};

export const useMemo$ = wrap(useMemo);

export const App = qComponent$((props) => {
    const state = createStore({count: 0});
    useMemo$(() => {
        console.log(state.count);
    });
    return onRender$(() => (
        <div>{state.count}</div>
    ));
});

export const Lightweight = (props) => {
    useMemo$(() => {
        console.log(state.count);
    });
});
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_missing_custom_inlined_functions() {
    test_input!(TestInput {
        code: r#"
import { qComponent$ as Component, onRender$, createStore, wrap, useEffect } from '@builder.io/qwik';


export const useMemo$ = (qrt) => {
    useEffect(qrt);
};

export const App = qComponent$((props) => {
    const state = createStore({count: 0});
    useMemo$(() => {
        console.log(state.count);
    });
    return onRender$(() => (
        <div>{state.count}</div>
    ));
});
"#
.to_string(),
transpile: true,
..TestInput::default()
});
}

#[test]
fn example_skip_transform() {
    test_input!(TestInput {
        code: r#"
import { qComponent$ as Component, onRender$ as onRender } from '@builder.io/qwik';

export const handler = $(()=>console.log('hola'));

export const App = qComponent$((props) => {
    withStyle$('hola');
    return onRender$(() => (
        <div>{state.thing}</div>
    ));
});
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[test]
fn example_explicit_ext_transpile() {
    test_input!(TestInput {
        code: r#"
import { qComponent$, onRender$, withStyle$ } from '@builder.io/qwik';

export const App = qComponent$((props) => {
    withStyle$('hola');
    return onRender$(() => (
        <div></div>
    ));
});
"#
        .to_string(),
        transpile: true,
        explicity_extensions: true,
        ..TestInput::default()
    });
}

#[test]
fn example_explicit_ext_no_transpile() {
    test_input!(TestInput {
        code: r#"
import { qComponent$, onRender$, withStyle$ } from '@builder.io/qwik';

export const App = qComponent$((props) => {
    withStyle$('hola');
    return onRender$(() => (
        <div></div>
    ));
});
"#
        .to_string(),
        explicity_extensions: true,
        ..TestInput::default()
    });
}

#[test]
fn issue_150() {
    test_input!(TestInput {
        code: r#"
import { component$, onRender$ } from '@builder.io/qwik';

export const Greeter = component$(() => {
    onRender$(() => {
    return (
        <div/>
    )
    });
});

const d = onRender$(()=>console.log('thing'));
"#
        .to_string(),
        transpile: true,
        ..TestInput::default()
    });
}

#[cfg(target_os = "windows")]
#[test]
fn issue_188() {
    let res = test_input!({
        filename: r"components\apps\apps.tsx".to_string(),
        root_dir: r"C:\users\apps".to_string(),
        code: r#"
import { component$, onRender$ } from '@builder.io/qwik';

export const Greeter = component$(() => {
    onRender$(() => {
    return (
        <div/>
    )
    });
});

const d = onRender$(()=>console.log('thing'));
"#
        .to_string(),
        transpile: true,
        snapshot: false,
    })
    .unwrap();
    let last_module = res.modules.last().unwrap();
    assert_eq!(last_module.path, r"C:/users/apps/components/apps/apps.tsx")
}

struct TestInput {
    pub code: String,
    pub filename: String,
    pub root_dir: String,
    pub entry_strategy: EntryStrategy,
    pub minify: MinifyMode,
    pub transpile: bool,
    pub explicity_extensions: bool,
    pub snapshot: bool,
}

impl TestInput {
    pub fn default() -> Self {
        Self {
            filename: "test.tsx".to_string(),
            root_dir: "/user/qwik/src/".to_string(),
            code: "/user/qwik/src/".to_string(),
            entry_strategy: EntryStrategy::Hook,
            minify: MinifyMode::Simplify,
            transpile: false,
            explicity_extensions: false,
            snapshot: true,
        }
    }
}

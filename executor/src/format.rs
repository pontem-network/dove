use std::fmt::Write;

use crate::explain::{
    StepExecutionResult, AddressResourceChanges, ExplainedTransactionEffects,
    StepResultInfo,
};

fn indent(num: usize) -> String {
    const STEP_INDENT: &str = "    ";
    std::iter::repeat(STEP_INDENT).take(num).collect()
}

fn format_error(out: &mut String, error: String) {
    write!(out, "{}", textwrap::indent(&error, &indent(1))).unwrap()
}

fn format_effects(out: &mut String, effects: ExplainedTransactionEffects) {
    for changes in effects.resources() {
        let AddressResourceChanges { address, changes } = changes;
        write!(out, "{}", textwrap::indent(address, &indent(1))).unwrap();
        for change in changes {
            write!(out, "{}", textwrap::indent(&change.to_string(), &indent(2))).unwrap();
        }
    }
    if !effects.events().is_empty() {
        write!(out, "{}", textwrap::indent("Events:", &indent(2))).unwrap();
        for event_change in effects.events() {
            write!(out, "{}", textwrap::indent(&event_change, &indent(3))).unwrap();
        }
    }
}

fn format_exec_status(step_exec_result: &StepExecutionResult) -> String {
    match step_exec_result {
        StepExecutionResult::Error(_) => "failed",
        _ => "ok",
    }
    .to_string()
}

pub fn format_step_result(
    step_result: StepResultInfo,
    verbose: bool,
    show_stats: bool,
) -> String {
    let mut out = String::new();
    let (name, gas, writeset_size, step_result) = step_result;

    let status = format_exec_status(&step_result);
    writeln!(&mut out, "{} ...... {}", name, status).unwrap();

    if show_stats {
        writeln!(
            &mut out,
            "[gas: {}, writeset bytes: {}]",
            gas, writeset_size
        )
        .unwrap();
    }

    match step_result {
        StepExecutionResult::Error(error) => format_error(&mut out, error),
        StepExecutionResult::ExpectedError(error) => {
            if verbose {
                format_error(&mut out, error)
            }
        }
        StepExecutionResult::Success(effects) => {
            if verbose {
                format_effects(&mut out, effects);
            }
        }
    }
    out
}

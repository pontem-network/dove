use std::fmt::Write;

use crate::explain::{
    StepExecutionResult, AddressResourceChanges, ResourceChange, ExplainedTransactionEffects,
    StepResultInfo,
};

const STEP_INDENT: &str = "    ";

fn indent(num: usize) -> String {
    let mut indent = String::new();
    for _ in 0..num {
        indent += STEP_INDENT;
    }
    indent
}

fn formatted_resource_change(change: &ResourceChange) -> String {
    let ResourceChange(ty, val) = change;
    match val {
        Some(val) => format!("{} =\n    {}", ty, val),
        None => ty.to_string(),
    }
}

fn format_error(out: &mut String, error: String) {
    write!(out, "{}", textwrap::indent(&error, &indent(1))).unwrap()
}

fn format_effects(out: &mut String, effects: ExplainedTransactionEffects) {
    for changes in effects.resources() {
        let AddressResourceChanges { address, changes } = changes;
        write!(out, "{}", textwrap::indent(address, &indent(1))).unwrap();
        for (operation, change) in changes {
            write!(
                out,
                "{}",
                textwrap::indent(
                    &format!("{} {}", operation, formatted_resource_change(change)),
                    &indent(2)
                )
            )
            .unwrap();
        }
    }
    if !effects.events().is_empty() {
        write!(out, "{}", textwrap::indent("Events:", &indent(2))).unwrap();
        for event_change in effects.events() {
            write!(
                out,
                "{}",
                textwrap::indent(&formatted_resource_change(event_change), &indent(3))
            )
            .unwrap();
        }
    }
}

fn format_exec_status(step_exec_result: &StepExecutionResult) -> String {
    match step_exec_result {
        StepExecutionResult::Error(_) => "FAILED",
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

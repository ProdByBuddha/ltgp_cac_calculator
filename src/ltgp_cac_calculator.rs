// Rust CLI LTGP:CAC + CAC/CFA Decision Tree Calculator (Human-Readable)
// Now supports an interactive guided form (use --interactive or run with no args).
// Build & run examples:
//   cargo run -- --cac 500.0 --cfa 200.0 --ltgp 2500.0 --early-gp-rate 50.0 --period days
//   cargo run -- --interactive

use clap::Parser;
use std::io::{self, Write};

/// Human-readable calculator that evaluates unit economics and cash dynamics.
#[derive(Parser, Debug)]
#[command(author, version, about = "LTGP:CAC calculator with an interactive guided form.", long_about = None)]
struct Args {
    /// Launch an interactive guided form to enter inputs
    #[arg(long, short = 'i', default_value_t = false)]
    interactive: bool,

    /// How much it costs you to acquire a client (CAC) in dollars
    #[arg(long)]
    cac: Option<f64>,

    /// How much money the client gives you upfront (CFA) in dollars
    #[arg(long)]
    cfa: Option<f64>,

    /// Lifetime Gross Profit you expect from this client (LTGP) in dollars
    #[arg(long)]
    ltgp: Option<f64>,

    /// How much profit you earn from this client per period at the start
    #[arg(long)]
    early_gp_rate: Option<f64>,

    /// Period unit for payback period output: days | weeks | months | years
    #[arg(long)]
    period: Option<String>,

    /// Consider CAC 'low' if CAC < threshold_fraction * LTGP (e.g., 0.10 = 10%)
    #[arg(long)]
    low_cac_fraction: Option<f64>,
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_money_like(s: &str) -> Option<f64> {
    let cleaned = s.replace(",", "").replace("$", "").trim().to_string();
    if cleaned.is_empty() { return None; }
    cleaned.parse::<f64>().ok()
}

fn prompt_f64_with_context(title: &str, what: &str, where_how: &str, why: &str, who: &str, prompt: &str, default: Option<f64>) -> f64 {
    loop {
        println!("\n{}", title);
        println!("• What it is: {}", what);
        println!("• Where/how to get it: {}", where_how);
        println!("• Why it matters: {}", why);
        println!("• Who it applies to: {}", who);
        let default_hint = default.map(|d| format!(" [default: {:.2}]", d)).unwrap_or_default();
        let input = read_line(&format!("{}{}: ", prompt, default_hint)).unwrap_or_default();
        if input.is_empty() {
            if let Some(d) = default { return d.max(0.0); }
        }
        if let Some(v) = parse_money_like(&input) {
            if v.is_finite() { return v.max(0.0); }
        }
        println!("Please enter a valid number (e.g., 500, 2500.75).");
    }
}

fn prompt_choice_with_context(title: &str, what: &str, where_how: &str, why: &str, who: &str, prompt: &str, choices: &[&str], default: &str) -> String {
    loop {
        println!("\n{}", title);
        println!("• What it is: {}", what);
        println!("• Where/how to choose: {}", where_how);
        println!("• Why it matters: {}", why);
        println!("• Who it applies to: {}", who);
        println!("Options: {}", choices.join(", "));
        let input = read_line(&format!("{} [default: {}]: ", prompt, default)).unwrap_or_default();
        let choice = if input.trim().is_empty() { default.to_string() } else { input.trim().to_lowercase() };
        if choices.iter().any(|c| c.eq_ignore_ascii_case(&choice)) { return choice; }
        println!("Please enter one of: {}", choices.join(", "));
    }
}

fn maybe_interactive_collect(args: &Args) -> (f64, f64, f64, f64, String, f64) {
    // Defaults when prompting interactively
    let default_period = "days".to_string();
    let default_low_frac = 0.10_f64;

    // If interactive flag is set OR any required value is missing, prompt.
    let need_interactive = args.interactive
        || args.cac.is_none()
        || args.ltgp.is_none()
        || args.cfa.is_none()
        || args.early_gp_rate.is_none()
        || args.period.is_none()
        || args.low_cac_fraction.is_none();

    if need_interactive {
        println!("\nWelcome! This guided form will help you estimate growth economics.\nYou can press Enter to accept defaults where shown.\n");

        let cac = args.cac.unwrap_or_else(|| prompt_f64_with_context(
            "Customer Acquisition Cost (CAC) — dollars per new customer",
            "The average fully-loaded cost to acquire one new customer (ads, sales commissions, SDR/AE time, agency fees, attributable tooling).",
            "From finance or growth analytics: take sales+marketing spend for a period and divide by the number of new customers acquired in that period.",
            "Determines how much cash you invest upfront and affects payback and ROI.",
            "Any business acquiring customers (SaaS, e‑commerce, services, marketplaces).",
            "Enter CAC in dollars",
            None,
        ));

        let cfa = args.cfa.unwrap_or_else(|| prompt_f64_with_context(
            "Customer Funds Upfront (CFA) — upfront cash from the customer",
            "Cash collected at or before acquisition: deposits, setup fees, prepayments, first invoice paid upfront.",
            "From pricing/billing: look at typical cash collected at purchase or at contract signature.",
            "Offsets CAC, lowering your net cash outlay and risk while speeding up payback.",
            "Businesses that collect money upfront. If you don’t, enter 0.",
            "Enter CFA in dollars",
            Some(0.0),
        ));

        let ltgp = args.ltgp.unwrap_or_else(|| prompt_f64_with_context(
            "Lifetime Gross Profit (LTGP) — total gross profit per customer",
            "Sum of (revenue − cost of goods sold) you expect over the customer’s lifetime.",
            "From cohort LTV or unit economics: monthly gross profit × expected lifetime (months), or lifetime revenue × gross margin.",
            "Primary measure of value; used to judge whether CAC is justified.",
            "The segment/cohort you’re modeling. Use a conservative estimate.",
            "Enter LTGP in dollars",
            None,
        ));

        let early_gp_rate = args.early_gp_rate.unwrap_or_else(|| prompt_f64_with_context(
            "Early Gross Profit Rate — profit earned per chosen period at the start",
            "Average gross profit per chosen period (e.g., per week) in the early customer lifecycle.",
            "From recent transactions: compute average contribution per period during the first few periods.",
            "Used to estimate how quickly you recover your upfront cash (payback period).",
            "Applies to your early lifecycle; if unknown, you can leave it blank to skip payback.",
            "Enter early gross profit per period",
            Some(0.0),
        ));

        let period = args.period.clone().unwrap_or_else(|| prompt_choice_with_context(
            "Period Unit — time unit used for the payback estimate",
            "The unit of time you want the payback estimate expressed in.",
            "Choose the unit that matches how you measure early profit (e.g., if early GP is weekly, choose weeks).",
            "Ensures the payback figure is in a meaningful unit.",
            "Anyone estimating payback.",
            "Choose one of: days, weeks, months, years",
            &["days", "weeks", "months", "years"],
            &default_period,
        ));

        let low_cac_fraction = args.low_cac_fraction.unwrap_or_else(|| prompt_f64_with_context(
            "Low CAC Threshold — fraction of LTGP considered ‘low CAC’",
            "A heuristic boundary: CAC < (threshold × LTGP).",
            "Use 0.10 (10%) by default; adjust to your risk tolerance and capital availability.",
            "Affects the quadrant label and qualitative guidance.",
            "Anyone using the quadrant classification.",
            "Enter threshold as a fraction (e.g., 0.10 for 10%)",
            Some(default_low_frac),
        ));

        (cac, cfa.max(0.0), ltgp, early_gp_rate.max(0.0), period.to_lowercase(), low_cac_fraction)
    } else {
        // Non-interactive path: all values provided
        (
            args.cac.unwrap(),
            args.cfa.unwrap_or(0.0).max(0.0),
            args.ltgp.unwrap(),
            args.early_gp_rate.unwrap_or(0.0).max(0.0),
            args.period.clone().unwrap_or_else(|| "days".to_string()).to_lowercase(),
            args.low_cac_fraction.unwrap_or(0.10),
        )
    }
}

fn main() {
    let args = Args::parse();

    let (cac, cfa, ltgp, early_gp, period, low_cac_fraction) = maybe_interactive_collect(&args);
    let low_cac_thresh = (low_cac_fraction.max(0.0)).min(1.0) * ltgp;

    // Net cash you actually spend (CAC minus what the client covers upfront)
    let net_outlay = (cac - cfa).max(0.0);

    // Lifetime return ratio
    let ratio = if cac > 0.0 { ltgp / cac } else { f64::INFINITY };

    // CAC classification
    let cac_label = if cac <= low_cac_thresh {
        "Low CAC (cheap to acquire a customer)"
    } else {
        "High CAC (expensive to acquire a customer)"
    };

    // CFA classification
    let cfa_label = if cfa >= cac * 0.5 {
        "High CFA (customer covers much of your cost upfront)"
    } else {
        "Low CFA (customer covers little upfront)"
    };

    // Quadrant placement
    let quadrant = match (cac <= low_cac_thresh, cfa >= cac * 0.5) {
        (true, true) => "Self-Funding Growth: customers pay for themselves upfront.",
        (true, false) => "Cash-Light Efficiency: customers are cheap to get, but you need some working capital.",
        (false, true) => "Deferred-Cash Risk: customers are expensive, but upfront payments soften the blow.",
        (false, false) => "Capital-Intensive Trap: customers are expensive and pay little upfront; very risky.",
    };

    // Verdict based on ratio and net outlay
    let verdict = if ratio <= 3.0 {
        if net_outlay == 0.0 {
            "Warning: Clients cover acquisition costs upfront, but long-term profits are too small (LTGP:CAC ≤ 3)."
        } else {
            "Unsustainable: You spend real money upfront and lifetime profits don’t justify it (LTGP:CAC ≤ 3)."
        }
    } else {
        if net_outlay == 0.0 {
            "Excellent: Clients fully finance their own acquisition and profits are healthy (LTGP:CAC > 3)."
        } else if cac <= low_cac_thresh {
            "Good: Profitable clients with quick payback; you just need a little cash buffer."
        } else if cfa >= cac * 0.5 {
            "Caution: Profitable clients, but growth is slower because they are costly to acquire."
        } else {
            "Fragile: Profitable on paper, but requires heavy upfront spending and is hard to scale safely."
        }
    };

    // Payback period estimate
    let ppd_est = if early_gp > 0.0 { Some(net_outlay / early_gp) } else { None };

    println!("\n=== Growth Model Evaluation ===\n");
    println!("You spend about ${:.2} to acquire a customer.", cac);
    println!("The customer gives you about ${:.2} upfront.", cfa);
    println!("Over their lifetime, you expect to make ${:.2} in gross profit.", ltgp);
    println!("\nThat means:");
    println!(" - Net cash you actually lay out upfront: ${:.2}.", net_outlay);
    println!(" - Lifetime return ratio (LTGP divided by CAC): {:.2}.", ratio);
    println!(" - CAC classification: {}", cac_label);
    println!(" - CFA classification: {}", cfa_label);
    println!(" - Quadrant: {}", quadrant);

    println!("\nVerdict: {}", verdict);

    match ppd_est {
        Some(value) => {
            println!("\nEstimated payback period: {:.2} {} (≈ {:.1} days).",
                value,
                &period,
                match period.as_str() {
                    "days" => value,
                    "weeks" => value * 7.0,
                    "months" => value * 30.0,
                    "years" => value * 365.0,
                    _ => value,
                }
            );
        }
        None => println!("\nPayback period could not be estimated. Provide --early-gp-rate to calculate it."),
    }

    println!("\nNotes:");
    println!(" - A lifetime return ratio above 3 means clients are worth it in the long run.");
    println!(" - If net outlay is zero, clients are financing their own acquisition.");
    println!(" - Low CAC and High CFA together create the safest and fastest growth.");
}

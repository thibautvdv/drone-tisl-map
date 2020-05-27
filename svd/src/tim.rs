//! TIM peripheral patches.

use anyhow::Result;
use drone_svd::Device;

/// Remove reserved fields in TIMER registers.
pub fn remove_reserved(dev: &mut Device) -> Result<()> {
    dev.periph("GPTIMER0").reg("PP").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBPV").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TAPV").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBPS").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TAPS").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBPMR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TAPMR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBPR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TAPR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBMATCHR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("TBILR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("ICR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("ICR").remove_field("Reserved16");
    dev.periph("GPTIMER0").reg("ICR").remove_field("Reserved8");
    dev.periph("GPTIMER0").reg("ICR").remove_field("Reserved3");
    dev.periph("GPTIMER0").reg("MIS").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("MIS").remove_field("Reserved8");
    dev.periph("GPTIMER0").reg("MIS").remove_field("Reserved3");
    dev.periph("GPTIMER0").reg("RIS").remove_field("Reserved16");
    dev.periph("GPTIMER0").reg("RIS").remove_field("Reserved8");
    dev.periph("GPTIMER0").reg("RIS").remove_field("Reserved3");
    dev.periph("GPTIMER0").reg("IMR").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("IMR").remove_field("Reserved8");
    dev.periph("GPTIMER0").reg("IMR").remove_field("Reserved3");
    dev.periph("GPTIMER0").reg("SYNC").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("CTL").remove_field("Reserved32");
    dev.periph("GPTIMER0").reg("CTL").remove_field("Reserved13");
    dev.periph("GPTIMER0").reg("CTL").remove_field("Reserved8");
    dev.periph("GPTIMER0").reg("CTL").remove_field("Reserved4");
    dev.periph("GPTIMER0").reg("TBMR").remove_field("Reserved12");
    dev.periph("GPTIMER0").reg("TAMR").remove_field("Reserved12");
    dev.periph("GPTIMER0").reg("TBV").remove_field("RESERVED");
    dev.periph("GPTIMER0").reg("CFG").remove_field("Reserved");

    dev.periph("GPTIMER1").reg("PP").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBPV").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TAPV").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBPS").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TAPS").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBPMR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TAPMR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBPR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TAPR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBMATCHR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("TBILR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("ICR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("ICR").remove_field("Reserved16");
    dev.periph("GPTIMER1").reg("ICR").remove_field("Reserved8");
    dev.periph("GPTIMER1").reg("ICR").remove_field("Reserved3");
    dev.periph("GPTIMER1").reg("MIS").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("MIS").remove_field("Reserved8");
    dev.periph("GPTIMER1").reg("MIS").remove_field("Reserved3");
    dev.periph("GPTIMER1").reg("RIS").remove_field("Reserved16");
    dev.periph("GPTIMER1").reg("RIS").remove_field("Reserved8");
    dev.periph("GPTIMER1").reg("RIS").remove_field("Reserved3");
    dev.periph("GPTIMER1").reg("IMR").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("IMR").remove_field("Reserved8");
    dev.periph("GPTIMER1").reg("IMR").remove_field("Reserved3");
    dev.periph("GPTIMER1").reg("SYNC").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("CTL").remove_field("Reserved32");
    dev.periph("GPTIMER1").reg("CTL").remove_field("Reserved13");
    dev.periph("GPTIMER1").reg("CTL").remove_field("Reserved8");
    dev.periph("GPTIMER1").reg("CTL").remove_field("Reserved4");
    dev.periph("GPTIMER1").reg("TBMR").remove_field("Reserved12");
    dev.periph("GPTIMER1").reg("TAMR").remove_field("Reserved12");
    dev.periph("GPTIMER1").reg("TBV").remove_field("RESERVED");
    dev.periph("GPTIMER1").reg("CFG").remove_field("Reserved");

    dev.periph("GPTIMER2").reg("PP").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBPV").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TAPV").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBPS").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TAPS").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBPMR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TAPMR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBPR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TAPR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBMATCHR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("TBILR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("ICR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("ICR").remove_field("Reserved16");
    dev.periph("GPTIMER2").reg("ICR").remove_field("Reserved8");
    dev.periph("GPTIMER2").reg("ICR").remove_field("Reserved3");
    dev.periph("GPTIMER2").reg("MIS").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("MIS").remove_field("Reserved8");
    dev.periph("GPTIMER2").reg("MIS").remove_field("Reserved3");
    dev.periph("GPTIMER2").reg("RIS").remove_field("Reserved16");
    dev.periph("GPTIMER2").reg("RIS").remove_field("Reserved8");
    dev.periph("GPTIMER2").reg("RIS").remove_field("Reserved3");
    dev.periph("GPTIMER2").reg("IMR").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("IMR").remove_field("Reserved8");
    dev.periph("GPTIMER2").reg("IMR").remove_field("Reserved3");
    dev.periph("GPTIMER2").reg("SYNC").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("CTL").remove_field("Reserved32");
    dev.periph("GPTIMER2").reg("CTL").remove_field("Reserved13");
    dev.periph("GPTIMER2").reg("CTL").remove_field("Reserved8");
    dev.periph("GPTIMER2").reg("CTL").remove_field("Reserved4");
    dev.periph("GPTIMER2").reg("TBMR").remove_field("Reserved12");
    dev.periph("GPTIMER2").reg("TAMR").remove_field("Reserved12");
    dev.periph("GPTIMER2").reg("TBV").remove_field("RESERVED");
    dev.periph("GPTIMER2").reg("CFG").remove_field("Reserved");

    dev.periph("GPTIMER3").reg("PP").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBPV").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TAPV").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBPS").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TAPS").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBPMR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TAPMR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBPR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TAPR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBMATCHR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("TBILR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("ICR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("ICR").remove_field("Reserved16");
    dev.periph("GPTIMER3").reg("ICR").remove_field("Reserved8");
    dev.periph("GPTIMER3").reg("ICR").remove_field("Reserved3");
    dev.periph("GPTIMER3").reg("MIS").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("MIS").remove_field("Reserved8");
    dev.periph("GPTIMER3").reg("MIS").remove_field("Reserved3");
    dev.periph("GPTIMER3").reg("RIS").remove_field("Reserved16");
    dev.periph("GPTIMER3").reg("RIS").remove_field("Reserved8");
    dev.periph("GPTIMER3").reg("RIS").remove_field("Reserved3");
    dev.periph("GPTIMER3").reg("IMR").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("IMR").remove_field("Reserved8");
    dev.periph("GPTIMER3").reg("IMR").remove_field("Reserved3");
    dev.periph("GPTIMER3").reg("SYNC").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("CTL").remove_field("Reserved32");
    dev.periph("GPTIMER3").reg("CTL").remove_field("Reserved13");
    dev.periph("GPTIMER3").reg("CTL").remove_field("Reserved8");
    dev.periph("GPTIMER3").reg("CTL").remove_field("Reserved4");
    dev.periph("GPTIMER3").reg("TBMR").remove_field("Reserved12");
    dev.periph("GPTIMER3").reg("TAMR").remove_field("Reserved12");
    dev.periph("GPTIMER3").reg("TBV").remove_field("RESERVED");
    dev.periph("GPTIMER3").reg("CFG").remove_field("Reserved");
    Ok(())
}
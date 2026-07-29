use unity2::prelude::*;
use unity2::OptionalMethod;
use engage_il2cpp::app::IClassChange_ChangeJobDataMethods;
use engage_il2cpp::app::ClassChange_ChangeJobData;
use engage_il2cpp::app::Unit;
use engage_il2cpp::app::IUnitMethods;
use engage_il2cpp::app::IJobDataMethods;

#[skyline::hook(offset=0x019c6700)]
pub fn compare_sid_jid(this: ClassChange_ChangeJobData, unit: Unit, _method_info: OptionalMethod) -> bool {
    let sid_append: Il2CppString = format!("SID_{}", this.get_job().get_jid()).into();
    let as_normal = call_original!(this, unit, _method_info);
    if unit.has_skill_2(sid_append) == true {
        println!("SID found: {}", format!("SID_{}", this.get_job().get_jid().to_string()));
        this.set_is_gender(true);
        this.set_is_default_job(true);
        return as_normal;
    }
    else {
        this.set_is_gender(false);
        return false;
    }
}
#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hook!(compare_sid_jid);
}
use unity::prelude::*;
use engage::app::eventscript::EventScript;
use engage::EventScriptExt;
use engage::{app::{IPersonDataMethods, ISkillDataMethods, PersonData, SkillData}, moon_sharp::interpreter::DynValue};
extern "C" fn personskillscript(arg: Array<DynValue>, method_info: OptionalMethod){
  let person = arg.get_string(0);
  let sid = arg.get_string(1);
  let person_get = PersonData::get_pid(person);
  let skill_get = SkillData::get_sid(sid);
  IPersonDataMethods::set_common_sids(person_get, skill_get);
  //(xyz, SkillData_Categorys::person(), 0);
}
pub fn register(event: EventScript) {
    event.register_action("PersonAddSkill", personskillscript);
}

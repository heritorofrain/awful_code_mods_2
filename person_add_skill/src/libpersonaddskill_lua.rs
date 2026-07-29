use unity::prelude::*;
use engage::app::eventscript::EventScript;
use engage::EventScriptExt;
use engage::app::{SkillArray, SkillData_Categorys};
use engage::{app::{IPersonDataMethods, ISkillDataMethods, IUnitMethods}, moon_sharp::interpreter::DynValue};
extern "C" fn personskillscript(args: Array<DynValue>, method_info: OptionalMethod){
  let person = args.get_pid(0);
  let sid = args.get_sid(1);
  person.add_skill_2(sid, SkillArray::add_2_method_info(), SkillData_Categorys::person());
  //
}
pub fn register(event: EventScript) {
    event.register_action("PersonAddSkill", personskillscript);
}

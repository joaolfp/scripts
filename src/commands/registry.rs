use super::{
	AppCommand, clone_my_repo::CloneMyRepo, hoc_clone::HocClone, install_xcode::InstallXcode,
	releasor::Releasor, rust_project::CreateRustProject, upgrade_mise::UpgradeMise,
};

pub fn all() -> Vec<Box<dyn AppCommand>> {
	vec![
		Box::new(HocClone),
		Box::new(Releasor),
		Box::new(CreateRustProject),
		Box::new(CloneMyRepo),
		Box::new(InstallXcode),
		Box::new(UpgradeMise),
	]
}

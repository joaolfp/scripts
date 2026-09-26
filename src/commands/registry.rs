use super::{
	AppCommand, clone_my_repo::CloneMyRepo, exit::Exit, hoc_clone::HocClone,
	install_claude::InstallClaude, install_kiro::InstallKiro, install_releasor::InstallReleasor,
	releasor::Releasor, rust_project::CreateRustProject, update_claude::UpdateClaude,
	update_rust::UpdateRust, upgrade_mise::UpgradeMise,
};

pub fn all() -> Vec<Box<dyn AppCommand>> {
	vec![
		Box::new(HocClone),
		Box::new(CloneMyRepo),
		Box::new(InstallReleasor),
		Box::new(InstallClaude),
		Box::new(InstallKiro),
		Box::new(UpgradeMise),
		Box::new(UpdateRust),
		Box::new(UpdateClaude),
		Box::new(Releasor),
		Box::new(CreateRustProject),
		Box::new(Exit),
	]
}

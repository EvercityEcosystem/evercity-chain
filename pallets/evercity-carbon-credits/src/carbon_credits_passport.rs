use crate::project::ProjectId;
use frame_support::{
    codec::{Decode, Encode},
    sp_runtime::RuntimeDebug,
};

/// Passport, that prooves, that an asset is a carbon credit asset
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq)]
pub struct CarbonCreditsPassport<AssetId>{
    asset_id: AssetId,
    project_id: CarbonCreditsOrigin,
    annual_report_index: u64,
}

impl<AssetId> CarbonCreditsPassport<AssetId> {
    pub fn new(asset_id: AssetId, project_id: ProjectId, annual_report_index: usize) -> Self {
        let annual_report_index_inner = annual_report_index as u64;

        CarbonCreditsPassport{
            asset_id,
            project_id: CarbonCreditsOrigin::CarbonProject(project_id),
            annual_report_index: annual_report_index_inner,
        }
    }

    pub fn new_from_bond(asset_id: AssetId, bond_id: [u8; 16]) -> Self {
        CarbonCreditsPassport {
            asset_id,
            project_id: CarbonCreditsOrigin::Bond(bond_id),
            annual_report_index: 0,
        }
    }

    pub fn get_project_id(&self) -> ProjectId { 
        match self.project_id {
            CarbonCreditsOrigin::CarbonProject(p_id) => p_id,
            CarbonCreditsOrigin::Bond(_) => ProjectId::default(),
        }
    }

    pub fn get_asset_id_ref(&self) -> &AssetId { 
        &self.asset_id
    }

    pub fn get_annual_report_index(&self) -> u64 {
        self.annual_report_index
    }

    pub fn get_last_report_index(&self) -> usize { 
        self.annual_report_index as usize
    }

    pub fn set_bond_id(&mut self, bond_id: [u8; 16]) {
        self.project_id = CarbonCreditsOrigin::Bond(bond_id);
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum CarbonCreditsOrigin {
    CarbonProject(ProjectId),
    Bond([u8; 16]),
}

impl Default for CarbonCreditsOrigin {
    fn default() -> Self {
        CarbonCreditsOrigin::CarbonProject(ProjectId::default())
    }
}
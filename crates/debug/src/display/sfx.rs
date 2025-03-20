use game::{
    cs::CSSfxImp,
    gxffx::{
        FxrListNode, FxrResourceContainer, FxrWrapper, GXFfxGraphicsResourceManager, GXFfxSceneCtrl,
    },
};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSSfxImp {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("CSSfxImp", TreeNodeFlags::empty()) {
            ui.text(format!("CSSfxImp: {:#01x}", self as *const _ as usize));
            ui.indent();
            self.scene_ctrl.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for GXFfxSceneCtrl {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Scene Ctrl", TreeNodeFlags::empty()) {
            ui.text(format!(
                "graphics_resource_manager: {:#01x}",
                self.graphics_resource_manager as *const _ as usize
            ));
            ui.indent();
            self.graphics_resource_manager.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for GXFfxGraphicsResourceManager {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Graphics Resource Manager", TreeNodeFlags::empty()) {
            ui.text(format!(
                "resource_container: {:#01x}",
                self.resource_container as *const _ as usize
            ));
            ui.indent();
            self.resource_container.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for FxrResourceContainer {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Resource Container", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!(
                "fxr_list_head: {:#01x}",
                self.fxr_list_head as *const _ as usize
            ));
            // self.fxr_list_head.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for FxrWrapper {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("FXR ptr: {:#01x}", self.fxr));
    }
}

impl DebugDisplay for FxrListNode {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Fxr List Node", TreeNodeFlags::empty()) {
            ui.text(format!("Fxr List Node: {:#01x}", self as *const _ as usize));

            ui.indent();
            ui.text(format!("ID: {:#01x}", self.id));
            self.next.render_debug(ui);
            self.prev.render_debug(ui);
            self.fxr_wrapper.render_debug(ui);
            ui.unindent();
        }
    }
}

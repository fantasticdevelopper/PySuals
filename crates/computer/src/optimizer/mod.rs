use crate::ast::*;
use super::CompilerConfig;

mod constant;
mod dce;
mod inline;
mod tree_shake;

use constant::ConstantFolder;
use dce::DeadCodeEliminator;
use inline::Inliner;
use tree_shake::TreeShaker;

pub struct Optimizer {
    constant_folder: ConstantFolder,
    dce: DeadCodeEliminator,
    inliner: Inliner,
    tree_shaker: TreeShaker,
    config: CompilerConfig,
}

impl Optimizer {
    pub fn new(config: &CompilerConfig) -> Self {
        Self {
            constant_folder: ConstantFolder::new(),
            dce: DeadCodeEliminator::new(),
            inliner: Inliner::new(),
            tree_shaker: TreeShaker::new(),
            config: config.clone(),
        }
    }
    
    pub fn optimize(&mut self, program: Program) -> Program {
        let mut program = program;
        
        if self.config.optimize {
            program = self.constant_folder.fold(program);
            program = self.dce.eliminate(program);
            program = self.inliner.inline(program);
            program = self.tree_shaker.shake(program);
        }
        
        program
    }
    
    pub fn should_optimize(&self) -> bool {
        self.config.optimize
    }
}

pub fn optimize(program: Program, config: &CompilerConfig) -> Program {
    let mut optimizer = Optimizer::new(config);
    optimizer.optimize(program)
}

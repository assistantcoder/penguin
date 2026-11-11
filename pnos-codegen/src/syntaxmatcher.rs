struct MakeNewVariableDefiner{
    list_vars: HashMap<String,Variable>,
    list_values: HashMap<String,pnos_ast::Types>,
    index: usize
}
impl MakeNewVariableDefiner{
    fn declare(&mut self,key: String,value: &pnos_ast::Types) -> Variable{
        let var = Variable::new(self.index);
        let key = key;
        self.index += 1;
        let _ = &self.list_vars.insert(key.clone(),var);
        let _ = self.list_values.entry(key).or_insert(value.clone());
        var
    }
}

struct MaterialProperties {
    pub albedo : vec![f32; 3 as usize];
    pub emission : vec![f32; 3 as usize];
    pub ray : ray::Ray;
}

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn get_enabled_features(_cargo_file: TokenStream) -> TokenStream {
    //https://www.reddit.com/r/rust/comments/834g53/list_of_conditional_compilation_features_at/
    let features_gen_code = r#"
        macro_rules! make_feat_list {
            ($($feat:expr),*) => {
                vec![ 
                    $(
                        #[cfg(feature = $feat)]
                        $feat,
                    )* 
                ]
            }
        }
        let enabled_features:Vec<&'static str> = make_feat_list!(FEATURES);
    "#;

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .features(cargo_metadata::CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    //Get string of features
    let root_package_features = format!(
        "{:?}",
        metadata
            .root_package()
            .unwrap()
            .features
            .keys()
            .cloned()
            .collect::<Vec<String>>()
    );

    //Without '[' in the  begin and ']' in the end
    let slice_features = &root_package_features[1..root_package_features.len() - 1];

    let features_gen_code = features_gen_code.replace("FEATURES", slice_features);

    features_gen_code.parse().unwrap()
}

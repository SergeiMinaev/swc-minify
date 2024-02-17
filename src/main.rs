use std::{ path::Path, sync::Arc, env };
use swc::{ self, BoolOrDataConfig, BoolConfig, config, try_with_handler };
use swc_common::{ GLOBALS, SourceMap };
use swc_ecma_ast::EsVersion;


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Specify input file.");
		return
	}
	let fname = &args[1];

	let cm = Arc::<SourceMap>::default();

	let opts = config::Options {
		config: config::Config {
			minify: BoolConfig::new(Some(true)),
			jsc: config::JscConfig {
				target: Some(EsVersion::EsNext),
				minify: Some(config::JsMinifyOptions {
					compress: BoolOrDataConfig::from_bool(true),
					mangle: BoolOrDataConfig::from_bool(true),
					..Default::default()
				}),
				..Default::default()
			},
			..Default::default()
		},
		..Default::default()
	};

	let c = swc::Compiler::new(cm.clone());
	let output = GLOBALS.set(&Default::default(), || {
			try_with_handler(cm.clone(), Default::default(), |handler| {
				let fm = cm.load_file(Path::new(fname)).expect("failed to load file");
				Ok(c.process_js_file(fm, handler, &opts).expect("failed to process file"))
			})
		}).unwrap();

	println!("{}", output.code);
}

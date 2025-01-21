/// Represents a individual programming language. Can be used to provide
/// information about the language, such as multi line comments, single line
/// comments, string literal syntax, whether a given language allows nesting
/// comments.
#[derive(Deserialize, Serialize)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum LanguageType {
    #[allow(missing_docs)] ABNF,
    #[allow(missing_docs)] Abap,
    #[allow(missing_docs)] ActionScript,
    #[allow(missing_docs)] Ada,
    #[allow(missing_docs)] Agda,
    #[allow(missing_docs)] Alex,
    #[allow(missing_docs)] Alloy,
    #[allow(missing_docs)] Arduino,
    #[allow(missing_docs)] AsciiDoc,
    #[allow(missing_docs)] Asn1,
    #[allow(missing_docs)] Asp,
    #[allow(missing_docs)] AspNet,
    #[allow(missing_docs)] Assembly,
    #[allow(missing_docs)] AssemblyGAS,
    #[allow(missing_docs)] AutoHotKey,
    #[allow(missing_docs)] Autoconf,
    #[allow(missing_docs)] Automake,
    #[allow(missing_docs)] Bash,
    #[allow(missing_docs)] Batch,
    #[allow(missing_docs)] Bean,
    #[allow(missing_docs)] BrightScript,
    #[allow(missing_docs)] C,
    #[allow(missing_docs)] CHeader,
    #[allow(missing_docs)] CMake,
    #[allow(missing_docs)] CSharp,
    #[allow(missing_docs)] CShell,
    #[allow(missing_docs)] Cabal,
    #[allow(missing_docs)] Cassius,
    #[allow(missing_docs)] Ceylon,
    #[allow(missing_docs)] Clojure,
    #[allow(missing_docs)] ClojureC,
    #[allow(missing_docs)] ClojureScript,
    #[allow(missing_docs)] Cobol,
    #[allow(missing_docs)] CodeQL,
    #[allow(missing_docs)] CoffeeScript,
    #[allow(missing_docs)] Cogent,
    #[allow(missing_docs)] ColdFusion,
    #[allow(missing_docs)] ColdFusionScript,
    #[allow(missing_docs)] Coq,
    #[allow(missing_docs)] Cpp,
    #[allow(missing_docs)] CppHeader,
    #[allow(missing_docs)] Crystal,
    #[allow(missing_docs)] Css,
    #[allow(missing_docs)] D,
    #[allow(missing_docs)] Daml,
    #[allow(missing_docs)] Dart,
    #[allow(missing_docs)] DeviceTree,
    #[allow(missing_docs)] Dhall,
    #[allow(missing_docs)] Dockerfile,
    #[allow(missing_docs)] DotNetResource,
    #[allow(missing_docs)] DreamMaker,
    #[allow(missing_docs)] Dust,
    #[allow(missing_docs)] Edn,
    #[allow(missing_docs)] Elisp,
    #[allow(missing_docs)] Elixir,
    #[allow(missing_docs)] Elm,
    #[allow(missing_docs)] Elvish,
    #[allow(missing_docs)] EmacsDevEnv,
    #[allow(missing_docs)] Emojicode,
    #[allow(missing_docs)] Erlang,
    #[allow(missing_docs)] FEN,
    #[allow(missing_docs)] FSharp,
    #[allow(missing_docs)] Fish,
    #[allow(missing_docs)] FlatBuffers,
    #[allow(missing_docs)] Forth,
    #[allow(missing_docs)] FortranLegacy,
    #[allow(missing_docs)] FortranModern,
    #[allow(missing_docs)] FreeMarker,
    #[allow(missing_docs)] Fstar,
    #[allow(missing_docs)] Futhark,
    #[allow(missing_docs)] GDB,
    #[allow(missing_docs)] GdScript,
    #[allow(missing_docs)] Gherkin,
    #[allow(missing_docs)] Gleam,
    #[allow(missing_docs)] Glsl,
    #[allow(missing_docs)] Go,
    #[allow(missing_docs)] Gohtml,
    #[allow(missing_docs)] Graphql,
    #[allow(missing_docs)] Groovy,
    #[allow(missing_docs)] Gwion,
    #[allow(missing_docs)] Hamlet,
    #[allow(missing_docs)] Handlebars,
    #[allow(missing_docs)] Happy,
    #[allow(missing_docs)] Haskell,
    #[allow(missing_docs)] Haxe,
    #[allow(missing_docs)] Hcl,
    #[allow(missing_docs)] Headache,
    #[allow(missing_docs)] Hex,
    #[allow(missing_docs)] Hlsl,
    #[allow(missing_docs)] HolyC,
    #[allow(missing_docs)] Html,
    #[allow(missing_docs)] Idris,
    #[allow(missing_docs)] Ini,
    #[allow(missing_docs)] IntelHex,
    #[allow(missing_docs)] Isabelle,
    #[allow(missing_docs)] Jai,
    #[allow(missing_docs)] Java,
    #[allow(missing_docs)] JavaScript,
    #[allow(missing_docs)] Json,
    #[allow(missing_docs)] Jsonnet,
    #[allow(missing_docs)] Jsx,
    #[allow(missing_docs)] Julia,
    #[allow(missing_docs)] Julius,
    #[allow(missing_docs)] Jupyter,
    #[allow(missing_docs)] K,
    #[allow(missing_docs)] KakouneScript,
    #[allow(missing_docs)] Kotlin,
    #[allow(missing_docs)] LLVM,
    #[allow(missing_docs)] Lean,
    #[allow(missing_docs)] Less,
    #[allow(missing_docs)] LinkerScript,
    #[allow(missing_docs)] Liquid,
    #[allow(missing_docs)] Lisp,
    #[allow(missing_docs)] LiveScript,
    #[allow(missing_docs)] Logtalk,
    #[allow(missing_docs)] Lua,
    #[allow(missing_docs)] Lucius,
    #[allow(missing_docs)] Madlang,
    #[allow(missing_docs)] Makefile,
    #[allow(missing_docs)] Markdown,
    #[allow(missing_docs)] Meson,
    #[allow(missing_docs)] Mint,
    #[allow(missing_docs)] ModuleDef,
    #[allow(missing_docs)] MoonScript,
    #[allow(missing_docs)] MsBuild,
    #[allow(missing_docs)] Mustache,
    #[allow(missing_docs)] Nim,
    #[allow(missing_docs)] Nix,
    #[allow(missing_docs)] NotQuitePerl,
    #[allow(missing_docs)] OCaml,
    #[allow(missing_docs)] ObjectiveC,
    #[allow(missing_docs)] ObjectiveCpp,
    #[allow(missing_docs)] Odin,
    #[allow(missing_docs)] OpenType,
    #[allow(missing_docs)] Org,
    #[allow(missing_docs)] Oz,
    #[allow(missing_docs)] PSL,
    #[allow(missing_docs)] Pan,
    #[allow(missing_docs)] Pascal,
    #[allow(missing_docs)] Perl,
    #[allow(missing_docs)] Perl6,
    #[allow(missing_docs)] Pest,
    #[allow(missing_docs)] Php,
    #[allow(missing_docs)] Polly,
    #[allow(missing_docs)] Pony,
    #[allow(missing_docs)] PostCss,
    #[allow(missing_docs)] PowerShell,
    #[allow(missing_docs)] Processing,
    #[allow(missing_docs)] Prolog,
    #[allow(missing_docs)] Protobuf,
    #[allow(missing_docs)] Pug,
    #[allow(missing_docs)] PureScript,
    #[allow(missing_docs)] Python,
    #[allow(missing_docs)] Q,
    #[allow(missing_docs)] Qcl,
    #[allow(missing_docs)] Qml,
    #[allow(missing_docs)] R,
    #[allow(missing_docs)] RON,
    #[allow(missing_docs)] RPMSpecfile,
    #[allow(missing_docs)] Racket,
    #[allow(missing_docs)] Rakefile,
    #[allow(missing_docs)] Razor,
    #[allow(missing_docs)] ReStructuredText,
    #[allow(missing_docs)] Renpy,
    #[allow(missing_docs)] Ruby,
    #[allow(missing_docs)] RubyHtml,
    #[allow(missing_docs)] Rust,
    #[allow(missing_docs)] SRecode,
    #[allow(missing_docs)] Sass,
    #[allow(missing_docs)] Scala,
    #[allow(missing_docs)] Scheme,
    #[allow(missing_docs)] Scons,
    #[allow(missing_docs)] Sh,
    #[allow(missing_docs)] Sml,
    #[allow(missing_docs)] Solidity,
    #[allow(missing_docs)] SpecmanE,
    #[allow(missing_docs)] Spice,
    #[allow(missing_docs)] Sql,
    #[allow(missing_docs)] Stan,
    #[allow(missing_docs)] Stratego,
    #[allow(missing_docs)] Stylus,
    #[allow(missing_docs)] Svelte,
    #[allow(missing_docs)] Svg,
    #[allow(missing_docs)] Swift,
    #[allow(missing_docs)] Swig,
    #[allow(missing_docs)] SystemVerilog,
    #[allow(missing_docs)] Tcl,
    #[allow(missing_docs)] Tera,
    #[allow(missing_docs)] Tex,
    #[allow(missing_docs)] Text,
    #[allow(missing_docs)] Thrift,
    #[allow(missing_docs)] Toml,
    #[allow(missing_docs)] Tsx,
    #[allow(missing_docs)] Ttcn,
    #[allow(missing_docs)] Twig,
    #[allow(missing_docs)] TypeScript,
    #[allow(missing_docs)] UnrealDeveloperMarkdown,
    #[allow(missing_docs)] UnrealPlugin,
    #[allow(missing_docs)] UnrealProject,
    #[allow(missing_docs)] UnrealScript,
    #[allow(missing_docs)] UnrealShader,
    #[allow(missing_docs)] UnrealShaderHeader,
    #[allow(missing_docs)] UrWeb,
    #[allow(missing_docs)] UrWebProject,
    #[allow(missing_docs)] VB6,
    #[allow(missing_docs)] VBScript,
    #[allow(missing_docs)] Vala,
    #[allow(missing_docs)] Velocity,
    #[allow(missing_docs)] Verilog,
    #[allow(missing_docs)] VerilogArgsFile,
    #[allow(missing_docs)] Vhdl,
    #[allow(missing_docs)] VimScript,
    #[allow(missing_docs)] VisualBasic,
    #[allow(missing_docs)] VisualStudioProject,
    #[allow(missing_docs)] VisualStudioSolution,
    #[allow(missing_docs)] Vue,
    #[allow(missing_docs)] WebAssembly,
    #[allow(missing_docs)] Wolfram,
    #[allow(missing_docs)] XSL,
    #[allow(missing_docs)] Xaml,
    #[allow(missing_docs)] XcodeConfig,
    #[allow(missing_docs)] Xml,
    #[allow(missing_docs)] Xtend,
    #[allow(missing_docs)] Yaml,
    #[allow(missing_docs)] Zig,
    #[allow(missing_docs)] Zsh,
    
}

impl LanguageType {

    /// Returns the display name of a language.
    ///
    /// ```
    /// # use tokei::*;
    /// let bash = LanguageType::Bash;
    ///
    /// assert_eq!(bash.name(), "BASH");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            ABNF => "ABNF",
            Abap => "ABAP",
            ActionScript => "ActionScript",
            Ada => "Ada",
            Agda => "Agda",
            Alex => "Alex",
            Alloy => "Alloy",
            Arduino => "Arduino C++",
            AsciiDoc => "AsciiDoc",
            Asn1 => "ASN.1",
            Asp => "ASP",
            AspNet => "ASP.NET",
            Assembly => "Assembly",
            AssemblyGAS => "GNU Style Assembly",
            AutoHotKey => "AutoHotKey",
            Autoconf => "Autoconf",
            Automake => "Automake",
            Bash => "BASH",
            Batch => "Batch",
            Bean => "Bean",
            BrightScript => "BrightScript",
            C => "C",
            CHeader => "C Header",
            CMake => "CMake",
            CSharp => "C#",
            CShell => "C Shell",
            Cabal => "Cabal",
            Cassius => "Cassius",
            Ceylon => "Ceylon",
            Clojure => "Clojure",
            ClojureC => "ClojureC",
            ClojureScript => "ClojureScript",
            Cobol => "COBOL",
            CodeQL => "CodeQL",
            CoffeeScript => "CoffeeScript",
            Cogent => "Cogent",
            ColdFusion => "ColdFusion",
            ColdFusionScript => "ColdFusion CFScript",
            Coq => "Coq",
            Cpp => "C++",
            CppHeader => "C++ Header",
            Crystal => "Crystal",
            Css => "CSS",
            D => "D",
            Daml => "DAML",
            Dart => "Dart",
            DeviceTree => "Device Tree",
            Dhall => "Dhall",
            Dockerfile => "Dockerfile",
            DotNetResource => ".NET Resource",
            DreamMaker => "Dream Maker",
            Dust => "Dust.js",
            Edn => "Edn",
            Elisp => "Emacs Lisp",
            Elixir => "Elixir",
            Elm => "Elm",
            Elvish => "Elvish",
            EmacsDevEnv => "Emacs Dev Env",
            Emojicode => "Emojicode",
            Erlang => "Erlang",
            FEN => "FEN",
            FSharp => "F#",
            Fish => "Fish",
            FlatBuffers => "FlatBuffers Schema",
            Forth => "Forth",
            FortranLegacy => "FORTRAN Legacy",
            FortranModern => "FORTRAN Modern",
            FreeMarker => "FreeMarker",
            Fstar => "F*",
            Futhark => "Futhark",
            GDB => "GDB Script",
            GdScript => "GDScript",
            Gherkin => "Gherkin (Cucumber)",
            Gleam => "Gleam",
            Glsl => "GLSL",
            Go => "Go",
            Gohtml => "Go HTML",
            Graphql => "GraphQL",
            Groovy => "Groovy",
            Gwion => "Gwion",
            Hamlet => "Hamlet",
            Handlebars => "Handlebars",
            Happy => "Happy",
            Haskell => "Haskell",
            Haxe => "Haxe",
            Hcl => "HCL",
            Headache => "Headache",
            Hex => "HEX",
            Hlsl => "HLSL",
            HolyC => "HolyC",
            Html => "HTML",
            Idris => "Idris",
            Ini => "INI",
            IntelHex => "Intel HEX",
            Isabelle => "Isabelle",
            Jai => "JAI",
            Java => "Java",
            JavaScript => "JavaScript",
            Json => "JSON",
            Jsonnet => "Jsonnet",
            Jsx => "JSX",
            Julia => "Julia",
            Julius => "Julius",
            Jupyter => "Jupyter Notebooks",
            K => "K",
            KakouneScript => "Kakoune script",
            Kotlin => "Kotlin",
            LLVM => "LLVM",
            Lean => "Lean",
            Less => "LESS",
            LinkerScript => "LD Script",
            Liquid => "Liquid",
            Lisp => "Lisp",
            LiveScript => "LiveScript",
            Logtalk => "Logtalk",
            Lua => "Lua",
            Lucius => "Lucius",
            Madlang => "Madlang",
            Makefile => "Makefile",
            Markdown => "Markdown",
            Meson => "Meson",
            Mint => "Mint",
            ModuleDef => "Module-Definition",
            MoonScript => "MoonScript",
            MsBuild => "MSBuild",
            Mustache => "Mustache",
            Nim => "Nim",
            Nix => "Nix",
            NotQuitePerl => "Not Quite Perl",
            OCaml => "OCaml",
            ObjectiveC => "Objective-C",
            ObjectiveCpp => "Objective-C++",
            Odin => "Odin",
            OpenType => "OpenType Feature File",
            Org => "Org",
            Oz => "Oz",
            PSL => "PSL Assertion",
            Pan => "Pan",
            Pascal => "Pascal",
            Perl => "Perl",
            Perl6 => "Rakudo",
            Pest => "Pest",
            Php => "PHP",
            Polly => "Polly",
            Pony => "Pony",
            PostCss => "PostCSS",
            PowerShell => "PowerShell",
            Processing => "Processing",
            Prolog => "Prolog",
            Protobuf => "Protocol Buffers",
            Pug => "Pug",
            PureScript => "PureScript",
            Python => "Python",
            Q => "Q",
            Qcl => "QCL",
            Qml => "QML",
            R => "R",
            RON => "Rusty Object Notation",
            RPMSpecfile => "RPM Specfile",
            Racket => "Racket",
            Rakefile => "Rakefile",
            Razor => "Razor",
            ReStructuredText => "ReStructuredText",
            Renpy => "Ren'Py",
            Ruby => "Ruby",
            RubyHtml => "Ruby HTML",
            Rust => "Rust",
            SRecode => "SRecode Template",
            Sass => "Sass",
            Scala => "Scala",
            Scheme => "Scheme",
            Scons => "Scons",
            Sh => "Shell",
            Sml => "Standard ML (SML)",
            Solidity => "Solidity",
            SpecmanE => "Specman e",
            Spice => "Spice Netlist",
            Sql => "SQL",
            Stan => "Stan",
            Stratego => "Stratego/XT",
            Stylus => "Stylus",
            Svelte => "Svelte",
            Svg => "SVG",
            Swift => "Swift",
            Swig => "SWIG",
            SystemVerilog => "SystemVerilog",
            Tcl => "TCL",
            Tera => "Tera",
            Tex => "TeX",
            Text => "Plain Text",
            Thrift => "Thrift",
            Toml => "TOML",
            Tsx => "TSX",
            Ttcn => "TTCN-3",
            Twig => "Twig",
            TypeScript => "TypeScript",
            UnrealDeveloperMarkdown => "Unreal Markdown",
            UnrealPlugin => "Unreal Plugin",
            UnrealProject => "Unreal Project",
            UnrealScript => "Unreal Script",
            UnrealShader => "Unreal Shader",
            UnrealShaderHeader => "Unreal Shader Header",
            UrWeb => "Ur/Web",
            UrWebProject => "Ur/Web Project",
            VB6 => "VB6",
            VBScript => "VBScript",
            Vala => "Vala",
            Velocity => "Apache Velocity",
            Verilog => "Verilog",
            VerilogArgsFile => "Verilog Args File",
            Vhdl => "VHDL",
            VimScript => "Vim script",
            VisualBasic => "Visual Basic",
            VisualStudioProject => "Visual Studio Project",
            VisualStudioSolution => "Visual Studio Solution",
            Vue => "Vue",
            WebAssembly => "WebAssembly",
            Wolfram => "Wolfram",
            XSL => "XSL",
            Xaml => "XAML",
            XcodeConfig => "Xcode Config",
            Xml => "XML",
            Xtend => "Xtend",
            Yaml => "YAML",
            Zig => "Zig",
            Zsh => "Zsh",
            
        }
    }

    pub(crate) fn _is_blank(self) -> bool {
        match self {
            ABNF => false,
            Abap => false,
            ActionScript => false,
            Ada => false,
            Agda => false,
            Alex => false,
            Alloy => false,
            Arduino => false,
            AsciiDoc => false,
            Asn1 => false,
            Asp => false,
            AspNet => false,
            Assembly => false,
            AssemblyGAS => false,
            AutoHotKey => false,
            Autoconf => false,
            Automake => false,
            Bash => false,
            Batch => false,
            Bean => false,
            BrightScript => false,
            C => false,
            CHeader => false,
            CMake => false,
            CSharp => false,
            CShell => false,
            Cabal => false,
            Cassius => false,
            Ceylon => false,
            Clojure => false,
            ClojureC => false,
            ClojureScript => false,
            Cobol => false,
            CodeQL => false,
            CoffeeScript => false,
            Cogent => false,
            ColdFusion => false,
            ColdFusionScript => false,
            Coq => false,
            Cpp => false,
            CppHeader => false,
            Crystal => false,
            Css => false,
            D => false,
            Daml => false,
            Dart => false,
            DeviceTree => false,
            Dhall => false,
            Dockerfile => false,
            DotNetResource => false,
            DreamMaker => false,
            Dust => false,
            Edn => false,
            Elisp => false,
            Elixir => false,
            Elm => false,
            Elvish => false,
            EmacsDevEnv => false,
            Emojicode => false,
            Erlang => false,
            FEN => true,
            FSharp => false,
            Fish => false,
            FlatBuffers => false,
            Forth => false,
            FortranLegacy => false,
            FortranModern => false,
            FreeMarker => false,
            Fstar => false,
            Futhark => false,
            GDB => false,
            GdScript => false,
            Gherkin => false,
            Gleam => false,
            Glsl => false,
            Go => false,
            Gohtml => false,
            Graphql => false,
            Groovy => false,
            Gwion => false,
            Hamlet => false,
            Handlebars => false,
            Happy => false,
            Haskell => false,
            Haxe => false,
            Hcl => false,
            Headache => false,
            Hex => true,
            Hlsl => false,
            HolyC => false,
            Html => false,
            Idris => false,
            Ini => false,
            IntelHex => true,
            Isabelle => false,
            Jai => false,
            Java => false,
            JavaScript => false,
            Json => true,
            Jsonnet => false,
            Jsx => false,
            Julia => false,
            Julius => false,
            Jupyter => false,
            K => false,
            KakouneScript => false,
            Kotlin => false,
            LLVM => false,
            Lean => false,
            Less => false,
            LinkerScript => false,
            Liquid => false,
            Lisp => false,
            LiveScript => false,
            Logtalk => false,
            Lua => false,
            Lucius => false,
            Madlang => false,
            Makefile => false,
            Markdown => false,
            Meson => false,
            Mint => true,
            ModuleDef => false,
            MoonScript => false,
            MsBuild => false,
            Mustache => false,
            Nim => false,
            Nix => false,
            NotQuitePerl => false,
            OCaml => false,
            ObjectiveC => false,
            ObjectiveCpp => false,
            Odin => false,
            OpenType => false,
            Org => false,
            Oz => false,
            PSL => false,
            Pan => false,
            Pascal => false,
            Perl => false,
            Perl6 => false,
            Pest => false,
            Php => false,
            Polly => false,
            Pony => false,
            PostCss => false,
            PowerShell => false,
            Processing => false,
            Prolog => false,
            Protobuf => false,
            Pug => false,
            PureScript => false,
            Python => false,
            Q => false,
            Qcl => false,
            Qml => false,
            R => false,
            RON => false,
            RPMSpecfile => false,
            Racket => false,
            Rakefile => false,
            Razor => false,
            ReStructuredText => true,
            Renpy => false,
            Ruby => false,
            RubyHtml => false,
            Rust => false,
            SRecode => false,
            Sass => false,
            Scala => false,
            Scheme => false,
            Scons => false,
            Sh => false,
            Sml => false,
            Solidity => false,
            SpecmanE => false,
            Spice => false,
            Sql => false,
            Stan => false,
            Stratego => false,
            Stylus => false,
            Svelte => false,
            Svg => false,
            Swift => false,
            Swig => false,
            SystemVerilog => false,
            Tcl => false,
            Tera => false,
            Tex => false,
            Text => false,
            Thrift => false,
            Toml => false,
            Tsx => false,
            Ttcn => false,
            Twig => false,
            TypeScript => false,
            UnrealDeveloperMarkdown => false,
            UnrealPlugin => true,
            UnrealProject => true,
            UnrealScript => false,
            UnrealShader => false,
            UnrealShaderHeader => false,
            UrWeb => false,
            UrWebProject => false,
            VB6 => false,
            VBScript => false,
            Vala => false,
            Velocity => false,
            Verilog => false,
            VerilogArgsFile => false,
            Vhdl => false,
            VimScript => false,
            VisualBasic => false,
            VisualStudioProject => false,
            VisualStudioSolution => true,
            Vue => false,
            WebAssembly => false,
            Wolfram => false,
            XSL => false,
            Xaml => false,
            XcodeConfig => false,
            Xml => false,
            Xtend => false,
            Yaml => false,
            Zig => false,
            Zsh => false,
            
        }
    }

    pub(crate) fn is_fortran(self) -> bool {
        self == LanguageType::FortranModern ||
        self == LanguageType::FortranLegacy
    }

    /// Returns whether the language is "literate", meaning that it considered
    /// to primarily be documentation and is counted primarily as comments
    /// rather than procedural code.
    pub fn is_literate(self) -> bool {
        match self {
            ABNF => false,
            Abap => false,
            ActionScript => false,
            Ada => false,
            Agda => false,
            Alex => false,
            Alloy => false,
            Arduino => false,
            AsciiDoc => false,
            Asn1 => false,
            Asp => false,
            AspNet => false,
            Assembly => false,
            AssemblyGAS => false,
            AutoHotKey => false,
            Autoconf => false,
            Automake => false,
            Bash => false,
            Batch => false,
            Bean => false,
            BrightScript => false,
            C => false,
            CHeader => false,
            CMake => false,
            CSharp => false,
            CShell => false,
            Cabal => false,
            Cassius => false,
            Ceylon => false,
            Clojure => false,
            ClojureC => false,
            ClojureScript => false,
            Cobol => false,
            CodeQL => false,
            CoffeeScript => false,
            Cogent => false,
            ColdFusion => false,
            ColdFusionScript => false,
            Coq => false,
            Cpp => false,
            CppHeader => false,
            Crystal => false,
            Css => false,
            D => false,
            Daml => false,
            Dart => false,
            DeviceTree => false,
            Dhall => false,
            Dockerfile => false,
            DotNetResource => false,
            DreamMaker => false,
            Dust => false,
            Edn => false,
            Elisp => false,
            Elixir => false,
            Elm => false,
            Elvish => false,
            EmacsDevEnv => false,
            Emojicode => false,
            Erlang => false,
            FEN => false,
            FSharp => false,
            Fish => false,
            FlatBuffers => false,
            Forth => false,
            FortranLegacy => false,
            FortranModern => false,
            FreeMarker => false,
            Fstar => false,
            Futhark => false,
            GDB => false,
            GdScript => false,
            Gherkin => false,
            Gleam => false,
            Glsl => false,
            Go => false,
            Gohtml => false,
            Graphql => false,
            Groovy => false,
            Gwion => false,
            Hamlet => false,
            Handlebars => false,
            Happy => false,
            Haskell => false,
            Haxe => false,
            Hcl => false,
            Headache => false,
            Hex => false,
            Hlsl => false,
            HolyC => false,
            Html => false,
            Idris => false,
            Ini => false,
            IntelHex => false,
            Isabelle => false,
            Jai => false,
            Java => false,
            JavaScript => false,
            Json => false,
            Jsonnet => false,
            Jsx => false,
            Julia => false,
            Julius => false,
            Jupyter => false,
            K => false,
            KakouneScript => false,
            Kotlin => false,
            LLVM => false,
            Lean => false,
            Less => false,
            LinkerScript => false,
            Liquid => false,
            Lisp => false,
            LiveScript => false,
            Logtalk => false,
            Lua => false,
            Lucius => false,
            Madlang => false,
            Makefile => false,
            Markdown => true,
            Meson => false,
            Mint => false,
            ModuleDef => false,
            MoonScript => false,
            MsBuild => false,
            Mustache => false,
            Nim => false,
            Nix => false,
            NotQuitePerl => false,
            OCaml => false,
            ObjectiveC => false,
            ObjectiveCpp => false,
            Odin => false,
            OpenType => false,
            Org => false,
            Oz => false,
            PSL => false,
            Pan => false,
            Pascal => false,
            Perl => false,
            Perl6 => false,
            Pest => false,
            Php => false,
            Polly => false,
            Pony => false,
            PostCss => false,
            PowerShell => false,
            Processing => false,
            Prolog => false,
            Protobuf => false,
            Pug => false,
            PureScript => false,
            Python => false,
            Q => false,
            Qcl => false,
            Qml => false,
            R => false,
            RON => false,
            RPMSpecfile => false,
            Racket => false,
            Rakefile => false,
            Razor => false,
            ReStructuredText => false,
            Renpy => false,
            Ruby => false,
            RubyHtml => false,
            Rust => false,
            SRecode => false,
            Sass => false,
            Scala => false,
            Scheme => false,
            Scons => false,
            Sh => false,
            Sml => false,
            Solidity => false,
            SpecmanE => false,
            Spice => false,
            Sql => false,
            Stan => false,
            Stratego => false,
            Stylus => false,
            Svelte => false,
            Svg => false,
            Swift => false,
            Swig => false,
            SystemVerilog => false,
            Tcl => false,
            Tera => false,
            Tex => false,
            Text => true,
            Thrift => false,
            Toml => false,
            Tsx => false,
            Ttcn => false,
            Twig => false,
            TypeScript => false,
            UnrealDeveloperMarkdown => false,
            UnrealPlugin => false,
            UnrealProject => false,
            UnrealScript => false,
            UnrealShader => false,
            UnrealShaderHeader => false,
            UrWeb => false,
            UrWebProject => false,
            VB6 => false,
            VBScript => false,
            Vala => false,
            Velocity => false,
            Verilog => false,
            VerilogArgsFile => false,
            Vhdl => false,
            VimScript => false,
            VisualBasic => false,
            VisualStudioProject => false,
            VisualStudioSolution => false,
            Vue => false,
            WebAssembly => false,
            Wolfram => false,
            XSL => false,
            Xaml => false,
            XcodeConfig => false,
            Xml => false,
            Xtend => false,
            Yaml => false,
            Zig => false,
            Zsh => false,
            
        }
    }

    /// Provides every variant in a Vec
    pub fn list() -> &'static [Self] {
        &[ABNF,Abap,ActionScript,Ada,Agda,Alex,Alloy,Arduino,AsciiDoc,Asn1,Asp,AspNet,Assembly,AssemblyGAS,AutoHotKey,Autoconf,Automake,Bash,Batch,Bean,BrightScript,C,CHeader,CMake,CSharp,CShell,Cabal,Cassius,Ceylon,Clojure,ClojureC,ClojureScript,Cobol,CodeQL,CoffeeScript,Cogent,ColdFusion,ColdFusionScript,Coq,Cpp,CppHeader,Crystal,Css,D,Daml,Dart,DeviceTree,Dhall,Dockerfile,DotNetResource,DreamMaker,Dust,Edn,Elisp,Elixir,Elm,Elvish,EmacsDevEnv,Emojicode,Erlang,FEN,FSharp,Fish,FlatBuffers,Forth,FortranLegacy,FortranModern,FreeMarker,Fstar,Futhark,GDB,GdScript,Gherkin,Gleam,Glsl,Go,Gohtml,Graphql,Groovy,Gwion,Hamlet,Handlebars,Happy,Haskell,Haxe,Hcl,Headache,Hex,Hlsl,HolyC,Html,Idris,Ini,IntelHex,Isabelle,Jai,Java,JavaScript,Json,Jsonnet,Jsx,Julia,Julius,Jupyter,K,KakouneScript,Kotlin,LLVM,Lean,Less,LinkerScript,Liquid,Lisp,LiveScript,Logtalk,Lua,Lucius,Madlang,Makefile,Markdown,Meson,Mint,ModuleDef,MoonScript,MsBuild,Mustache,Nim,Nix,NotQuitePerl,OCaml,ObjectiveC,ObjectiveCpp,Odin,OpenType,Org,Oz,PSL,Pan,Pascal,Perl,Perl6,Pest,Php,Polly,Pony,PostCss,PowerShell,Processing,Prolog,Protobuf,Pug,PureScript,Python,Q,Qcl,Qml,R,RON,RPMSpecfile,Racket,Rakefile,Razor,ReStructuredText,Renpy,Ruby,RubyHtml,Rust,SRecode,Sass,Scala,Scheme,Scons,Sh,Sml,Solidity,SpecmanE,Spice,Sql,Stan,Stratego,Stylus,Svelte,Svg,Swift,Swig,SystemVerilog,Tcl,Tera,Tex,Text,Thrift,Toml,Tsx,Ttcn,Twig,TypeScript,UnrealDeveloperMarkdown,UnrealPlugin,UnrealProject,UnrealScript,UnrealShader,UnrealShaderHeader,UrWeb,UrWebProject,VB6,VBScript,Vala,Velocity,Verilog,VerilogArgsFile,Vhdl,VimScript,VisualBasic,VisualStudioProject,VisualStudioSolution,Vue,WebAssembly,Wolfram,XSL,Xaml,XcodeConfig,Xml,Xtend,Yaml,Zig,Zsh,]
    }

    /// Returns the single line comments of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.line_comments(), &["//"]);
    /// ```
    pub fn line_comments(self) -> &'static [&'static str] {
        match self {
            ABNF => &[";",],
            Abap => &["*","\"",],
            ActionScript => &["//",],
            Ada => &["--",],
            Agda => &["--",],
            Alex => &[],
            Alloy => &["--","//",],
            Arduino => &["//",],
            AsciiDoc => &["//",],
            Asn1 => &["--",],
            Asp => &["'","REM",],
            AspNet => &[],
            Assembly => &[";",],
            AssemblyGAS => &["//",],
            AutoHotKey => &[";",],
            Autoconf => &["#","dnl",],
            Automake => &["#",],
            Bash => &["#",],
            Batch => &["REM","::",],
            Bean => &[";",],
            BrightScript => &["'","REM",],
            C => &["//",],
            CHeader => &["//",],
            CMake => &["#",],
            CSharp => &["//",],
            CShell => &["#",],
            Cabal => &["--",],
            Cassius => &["//",],
            Ceylon => &["//",],
            Clojure => &[";",],
            ClojureC => &[";",],
            ClojureScript => &[";",],
            Cobol => &["*",],
            CodeQL => &["//",],
            CoffeeScript => &["#",],
            Cogent => &["--",],
            ColdFusion => &[],
            ColdFusionScript => &["//",],
            Coq => &[],
            Cpp => &["//",],
            CppHeader => &["//",],
            Crystal => &["#",],
            Css => &["//",],
            D => &["//",],
            Daml => &["-- ",],
            Dart => &["//",],
            DeviceTree => &["//",],
            Dhall => &["--",],
            Dockerfile => &["#",],
            DotNetResource => &[],
            DreamMaker => &["//",],
            Dust => &[],
            Edn => &[";",],
            Elisp => &[";",],
            Elixir => &["#",],
            Elm => &["--",],
            Elvish => &["#",],
            EmacsDevEnv => &[";",],
            Emojicode => &["💭",],
            Erlang => &["%",],
            FEN => &[],
            FSharp => &["//",],
            Fish => &["#",],
            FlatBuffers => &["//",],
            Forth => &["\\",],
            FortranLegacy => &["c","C","!","*",],
            FortranModern => &["!",],
            FreeMarker => &[],
            Fstar => &["//",],
            Futhark => &["--",],
            GDB => &["#",],
            GdScript => &["#",],
            Gherkin => &["#",],
            Gleam => &["//","///","////",],
            Glsl => &["//",],
            Go => &["//",],
            Gohtml => &[],
            Graphql => &["#",],
            Groovy => &["//",],
            Gwion => &["#!",],
            Hamlet => &[],
            Handlebars => &[],
            Happy => &[],
            Haskell => &["--",],
            Haxe => &["//",],
            Hcl => &["#","//",],
            Headache => &["//",],
            Hex => &[],
            Hlsl => &["//",],
            HolyC => &["//",],
            Html => &[],
            Idris => &["--",],
            Ini => &[";","#",],
            IntelHex => &[],
            Isabelle => &["--",],
            Jai => &["//",],
            Java => &["//",],
            JavaScript => &["//",],
            Json => &[],
            Jsonnet => &["//","#",],
            Jsx => &["//",],
            Julia => &["#",],
            Julius => &["//",],
            Jupyter => &[],
            K => &["/",],
            KakouneScript => &["#",],
            Kotlin => &["//",],
            LLVM => &[";",],
            Lean => &["--",],
            Less => &["//",],
            LinkerScript => &["//",],
            Liquid => &[],
            Lisp => &[";",],
            LiveScript => &["#",],
            Logtalk => &["%",],
            Lua => &["--",],
            Lucius => &["//",],
            Madlang => &["#",],
            Makefile => &["#",],
            Markdown => &[],
            Meson => &["#",],
            Mint => &[],
            ModuleDef => &[";",],
            MoonScript => &["--",],
            MsBuild => &[],
            Mustache => &[],
            Nim => &["#",],
            Nix => &["#",],
            NotQuitePerl => &["#",],
            OCaml => &[],
            ObjectiveC => &["//",],
            ObjectiveCpp => &["//",],
            Odin => &["//",],
            OpenType => &["#",],
            Org => &["# ",],
            Oz => &["%",],
            PSL => &["//",],
            Pan => &["#",],
            Pascal => &["//",],
            Perl => &["#",],
            Perl6 => &["#",],
            Pest => &["//",],
            Php => &["#","//",],
            Polly => &[],
            Pony => &["//",],
            PostCss => &["//",],
            PowerShell => &["#",],
            Processing => &["//",],
            Prolog => &["%",],
            Protobuf => &["//",],
            Pug => &["//","//-",],
            PureScript => &["--",],
            Python => &["#",],
            Q => &["/",],
            Qcl => &["//",],
            Qml => &["//",],
            R => &["#",],
            RON => &["//",],
            RPMSpecfile => &["#",],
            Racket => &[";",],
            Rakefile => &["#",],
            Razor => &[],
            ReStructuredText => &[],
            Renpy => &["#",],
            Ruby => &["#",],
            RubyHtml => &[],
            Rust => &["//",],
            SRecode => &[";;",],
            Sass => &["//",],
            Scala => &["//",],
            Scheme => &[";",],
            Scons => &["#",],
            Sh => &["#",],
            Sml => &[],
            Solidity => &["//",],
            SpecmanE => &["--","//",],
            Spice => &["*",],
            Sql => &["--",],
            Stan => &["//","#",],
            Stratego => &["//",],
            Stylus => &["//",],
            Svelte => &[],
            Svg => &[],
            Swift => &["//",],
            Swig => &["//",],
            SystemVerilog => &["//",],
            Tcl => &["#",],
            Tera => &[],
            Tex => &["%",],
            Text => &[],
            Thrift => &["#","//",],
            Toml => &["#",],
            Tsx => &["//",],
            Ttcn => &["//",],
            Twig => &[],
            TypeScript => &["//",],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &["//",],
            UnrealShader => &["//",],
            UnrealShaderHeader => &["//",],
            UrWeb => &[],
            UrWebProject => &["#",],
            VB6 => &["'",],
            VBScript => &["'","REM",],
            Vala => &["//",],
            Velocity => &["##",],
            Verilog => &["//",],
            VerilogArgsFile => &[],
            Vhdl => &["--",],
            VimScript => &["\"",],
            VisualBasic => &["'",],
            VisualStudioProject => &[],
            VisualStudioSolution => &[],
            Vue => &["//",],
            WebAssembly => &[";;",],
            Wolfram => &[],
            XSL => &[],
            Xaml => &[],
            XcodeConfig => &["//",],
            Xml => &[],
            Xtend => &["//",],
            Yaml => &["#",],
            Zig => &["//",],
            Zsh => &["#",],
            
        }
    }

    /// Returns the single line comments of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.multi_line_comments(), &[("/*", "*/")]);
    /// ```
    pub fn multi_line_comments(self) -> &'static [(&'static str, &'static str)]
    {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &[("/*","*/",),],
            Ada => &[],
            Agda => &[("{-","-}",),],
            Alex => &[],
            Alloy => &[("/*","*/",),],
            Arduino => &[("/*","*/",),],
            AsciiDoc => &[("////","////",),],
            Asn1 => &[("/*","*/",),],
            Asp => &[],
            AspNet => &[("<!--","-->",),("<%--","-->",),],
            Assembly => &[],
            AssemblyGAS => &[("/*","*/",),],
            AutoHotKey => &[("/*","*/",),],
            Autoconf => &[],
            Automake => &[],
            Bash => &[],
            Batch => &[],
            Bean => &[],
            BrightScript => &[],
            C => &[("/*","*/",),],
            CHeader => &[("/*","*/",),],
            CMake => &[],
            CSharp => &[("/*","*/",),],
            CShell => &[],
            Cabal => &[("{-","-}",),],
            Cassius => &[("/*","*/",),],
            Ceylon => &[("/*","*/",),],
            Clojure => &[],
            ClojureC => &[],
            ClojureScript => &[],
            Cobol => &[],
            CodeQL => &[("/*","*/",),],
            CoffeeScript => &[("###","###",),],
            Cogent => &[],
            ColdFusion => &[("<!---","--->",),],
            ColdFusionScript => &[("/*","*/",),],
            Coq => &[("(*","*)",),],
            Cpp => &[("/*","*/",),],
            CppHeader => &[("/*","*/",),],
            Crystal => &[],
            Css => &[("/*","*/",),],
            D => &[("/*","*/",),],
            Daml => &[("{-","-}",),],
            Dart => &[("/*","*/",),],
            DeviceTree => &[("/*","*/",),],
            Dhall => &[("{-","-}",),],
            Dockerfile => &[],
            DotNetResource => &[("<!--","-->",),],
            DreamMaker => &[("/*","*/",),],
            Dust => &[("{!","!}",),],
            Edn => &[],
            Elisp => &[],
            Elixir => &[],
            Elm => &[("{-","-}",),],
            Elvish => &[],
            EmacsDevEnv => &[],
            Emojicode => &[("💭🔜","🔚💭",),("📗","📗",),("📘","📘",),],
            Erlang => &[],
            FEN => &[],
            FSharp => &[("(*","*)",),],
            Fish => &[],
            FlatBuffers => &[("/*","*/",),],
            Forth => &[("( ",")",),],
            FortranLegacy => &[],
            FortranModern => &[],
            FreeMarker => &[("<#--","-->",),],
            Fstar => &[("(*","*)",),],
            Futhark => &[],
            GDB => &[],
            GdScript => &[],
            Gherkin => &[],
            Gleam => &[],
            Glsl => &[("/*","*/",),],
            Go => &[("/*","*/",),],
            Gohtml => &[("<!--","-->",),("{{/*","*/}}",),],
            Graphql => &[],
            Groovy => &[("/*","*/",),],
            Gwion => &[],
            Hamlet => &[("<!--","-->",),],
            Handlebars => &[("<!--","-->",),("{{!","}}",),],
            Happy => &[],
            Haskell => &[("{-","-}",),],
            Haxe => &[("/*","*/",),],
            Hcl => &[("/*","*/",),],
            Headache => &[("/*","*/",),],
            Hex => &[],
            Hlsl => &[("/*","*/",),],
            HolyC => &[("/*","*/",),],
            Html => &[("<!--","-->",),],
            Idris => &[("{-","-}",),],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &[("{*","*}",),("(*","*)",),("‹","›",),("\\<open>","\\<close>",),],
            Jai => &[("/*","*/",),],
            Java => &[("/*","*/",),],
            JavaScript => &[("/*","*/",),],
            Json => &[],
            Jsonnet => &[("/*","*/",),],
            Jsx => &[("/*","*/",),],
            Julia => &[("#=","=#",),],
            Julius => &[("/*","*/",),],
            Jupyter => &[],
            K => &[],
            KakouneScript => &[],
            Kotlin => &[("/*","*/",),],
            LLVM => &[],
            Lean => &[("/-","-/",),],
            Less => &[("/*","*/",),],
            LinkerScript => &[("/*","*/",),],
            Liquid => &[("<!--","-->",),("{% comment %}","{% endcomment %}",),],
            Lisp => &[("#|","|#",),],
            LiveScript => &[("/*","*/",),],
            Logtalk => &[("/*","*/",),],
            Lua => &[("--[[","]]",),],
            Lucius => &[("/*","*/",),],
            Madlang => &[("{#","#}",),],
            Makefile => &[],
            Markdown => &[],
            Meson => &[],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &[],
            MsBuild => &[("<!--","-->",),],
            Mustache => &[("{{!","}}",),],
            Nim => &[],
            Nix => &[("/*","*/",),],
            NotQuitePerl => &[("=begin","=end",),],
            OCaml => &[("(*","*)",),],
            ObjectiveC => &[("/*","*/",),],
            ObjectiveCpp => &[("/*","*/",),],
            Odin => &[("/*","*/",),],
            OpenType => &[],
            Org => &[],
            Oz => &[("/*","*/",),],
            PSL => &[("/*","*/",),],
            Pan => &[],
            Pascal => &[("{","}",),("(*","*)",),],
            Perl => &[("=pod","=cut",),],
            Perl6 => &[("=begin","=end",),],
            Pest => &[],
            Php => &[("/*","*/",),],
            Polly => &[("<!--","-->",),],
            Pony => &[("/*","*/",),],
            PostCss => &[("/*","*/",),],
            PowerShell => &[("<#","#>",),],
            Processing => &[("/*","*/",),],
            Prolog => &[("/*","*/",),],
            Protobuf => &[],
            Pug => &[],
            PureScript => &[("{-","-}",),],
            Python => &[],
            Q => &[],
            Qcl => &[("/*","*/",),],
            Qml => &[("/*","*/",),],
            R => &[],
            RON => &[("/*","*/",),],
            RPMSpecfile => &[],
            Racket => &[("#|","|#",),],
            Rakefile => &[("=begin","=end",),],
            Razor => &[("<!--","-->",),("@*","*@",),],
            ReStructuredText => &[],
            Renpy => &[],
            Ruby => &[("=begin","=end",),],
            RubyHtml => &[("<!--","-->",),],
            Rust => &[("/*","*/",),],
            SRecode => &[],
            Sass => &[("/*","*/",),],
            Scala => &[("/*","*/",),],
            Scheme => &[("#|","|#",),],
            Scons => &[],
            Sh => &[],
            Sml => &[("(*","*)",),],
            Solidity => &[("/*","*/",),],
            SpecmanE => &[("'>","<'",),],
            Spice => &[],
            Sql => &[("/*","*/",),],
            Stan => &[("/*","*/",),],
            Stratego => &[("/*","*/",),],
            Stylus => &[("/*","*/",),],
            Svelte => &[("<!--","-->",),],
            Svg => &[("<!--","-->",),],
            Swift => &[("/*","*/",),],
            Swig => &[("/*","*/",),],
            SystemVerilog => &[("/*","*/",),],
            Tcl => &[],
            Tera => &[("<!--","-->",),("{#","#}",),],
            Tex => &[],
            Text => &[],
            Thrift => &[("/*","*/",),],
            Toml => &[],
            Tsx => &[("/*","*/",),],
            Ttcn => &[("/*","*/",),],
            Twig => &[("<!--","-->",),("{#","#}",),],
            TypeScript => &[("/*","*/",),],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &[("/*","*/",),],
            UnrealShader => &[("/*","*/",),],
            UnrealShaderHeader => &[("/*","*/",),],
            UrWeb => &[("(*","*)",),],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &[("/*","*/",),],
            Velocity => &[("#*","*#",),],
            Verilog => &[("/*","*/",),],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &[],
            VisualBasic => &[],
            VisualStudioProject => &[("<!--","-->",),],
            VisualStudioSolution => &[],
            Vue => &[("<!--","-->",),("/*","*/",),],
            WebAssembly => &[],
            Wolfram => &[("(*","*)",),],
            XSL => &[("<!--","-->",),],
            Xaml => &[("<!--","-->",),],
            XcodeConfig => &[],
            Xml => &[("<!--","-->",),],
            Xtend => &[("/*","*/",),],
            Yaml => &[],
            Zig => &[],
            Zsh => &[],
            
        }
    }


    /// Returns whether the language allows nested multi line comments.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert!(lang.allows_nested());
    /// ```
    pub fn allows_nested(self) -> bool {
        match self {
            ABNF => false,
            Abap => false,
            ActionScript => false,
            Ada => false,
            Agda => true,
            Alex => false,
            Alloy => false,
            Arduino => false,
            AsciiDoc => false,
            Asn1 => false,
            Asp => false,
            AspNet => false,
            Assembly => false,
            AssemblyGAS => false,
            AutoHotKey => false,
            Autoconf => false,
            Automake => false,
            Bash => false,
            Batch => false,
            Bean => false,
            BrightScript => false,
            C => false,
            CHeader => false,
            CMake => false,
            CSharp => false,
            CShell => false,
            Cabal => true,
            Cassius => false,
            Ceylon => false,
            Clojure => false,
            ClojureC => false,
            ClojureScript => false,
            Cobol => false,
            CodeQL => false,
            CoffeeScript => false,
            Cogent => false,
            ColdFusion => false,
            ColdFusionScript => false,
            Coq => false,
            Cpp => false,
            CppHeader => false,
            Crystal => false,
            Css => false,
            D => false,
            Daml => true,
            Dart => false,
            DeviceTree => false,
            Dhall => true,
            Dockerfile => false,
            DotNetResource => false,
            DreamMaker => true,
            Dust => false,
            Edn => false,
            Elisp => false,
            Elixir => false,
            Elm => true,
            Elvish => false,
            EmacsDevEnv => false,
            Emojicode => false,
            Erlang => false,
            FEN => false,
            FSharp => false,
            Fish => false,
            FlatBuffers => false,
            Forth => false,
            FortranLegacy => false,
            FortranModern => false,
            FreeMarker => false,
            Fstar => false,
            Futhark => false,
            GDB => false,
            GdScript => false,
            Gherkin => false,
            Gleam => false,
            Glsl => false,
            Go => false,
            Gohtml => false,
            Graphql => false,
            Groovy => false,
            Gwion => false,
            Hamlet => false,
            Handlebars => false,
            Happy => false,
            Haskell => true,
            Haxe => false,
            Hcl => false,
            Headache => false,
            Hex => false,
            Hlsl => false,
            HolyC => false,
            Html => false,
            Idris => true,
            Ini => false,
            IntelHex => false,
            Isabelle => false,
            Jai => true,
            Java => false,
            JavaScript => false,
            Json => false,
            Jsonnet => false,
            Jsx => false,
            Julia => true,
            Julius => false,
            Jupyter => false,
            K => true,
            KakouneScript => false,
            Kotlin => true,
            LLVM => false,
            Lean => true,
            Less => false,
            LinkerScript => false,
            Liquid => false,
            Lisp => true,
            LiveScript => false,
            Logtalk => false,
            Lua => false,
            Lucius => false,
            Madlang => false,
            Makefile => false,
            Markdown => false,
            Meson => false,
            Mint => false,
            ModuleDef => false,
            MoonScript => false,
            MsBuild => false,
            Mustache => false,
            Nim => false,
            Nix => false,
            NotQuitePerl => false,
            OCaml => false,
            ObjectiveC => false,
            ObjectiveCpp => false,
            Odin => false,
            OpenType => false,
            Org => false,
            Oz => false,
            PSL => false,
            Pan => false,
            Pascal => true,
            Perl => false,
            Perl6 => false,
            Pest => false,
            Php => false,
            Polly => false,
            Pony => false,
            PostCss => false,
            PowerShell => false,
            Processing => false,
            Prolog => false,
            Protobuf => false,
            Pug => false,
            PureScript => true,
            Python => false,
            Q => true,
            Qcl => false,
            Qml => false,
            R => false,
            RON => true,
            RPMSpecfile => false,
            Racket => true,
            Rakefile => false,
            Razor => false,
            ReStructuredText => false,
            Renpy => false,
            Ruby => false,
            RubyHtml => false,
            Rust => true,
            SRecode => false,
            Sass => false,
            Scala => false,
            Scheme => true,
            Scons => false,
            Sh => false,
            Sml => false,
            Solidity => false,
            SpecmanE => false,
            Spice => false,
            Sql => false,
            Stan => false,
            Stratego => false,
            Stylus => false,
            Svelte => false,
            Svg => false,
            Swift => true,
            Swig => true,
            SystemVerilog => false,
            Tcl => false,
            Tera => false,
            Tex => false,
            Text => false,
            Thrift => false,
            Toml => false,
            Tsx => false,
            Ttcn => false,
            Twig => false,
            TypeScript => false,
            UnrealDeveloperMarkdown => false,
            UnrealPlugin => false,
            UnrealProject => false,
            UnrealScript => false,
            UnrealShader => false,
            UnrealShaderHeader => false,
            UrWeb => false,
            UrWebProject => false,
            VB6 => false,
            VBScript => false,
            Vala => false,
            Velocity => false,
            Verilog => false,
            VerilogArgsFile => false,
            Vhdl => false,
            VimScript => false,
            VisualBasic => false,
            VisualStudioProject => false,
            VisualStudioSolution => false,
            Vue => false,
            WebAssembly => false,
            Wolfram => false,
            XSL => false,
            Xaml => false,
            XcodeConfig => false,
            Xml => false,
            Xtend => false,
            Yaml => false,
            Zig => false,
            Zsh => false,
            
        }
    }

    /// Returns what nested comments the language has. (Currently only D has
    /// any of this type.)
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::D;
    /// assert_eq!(lang.nested_comments(), &[("/+", "+/")]);
    /// ```
    pub fn nested_comments(self) -> &'static [(&'static str, &'static str)]
    {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &[],
            Ada => &[],
            Agda => &[],
            Alex => &[],
            Alloy => &[],
            Arduino => &[],
            AsciiDoc => &[],
            Asn1 => &[],
            Asp => &[],
            AspNet => &[],
            Assembly => &[],
            AssemblyGAS => &[],
            AutoHotKey => &[],
            Autoconf => &[],
            Automake => &[],
            Bash => &[],
            Batch => &[],
            Bean => &[],
            BrightScript => &[],
            C => &[],
            CHeader => &[],
            CMake => &[],
            CSharp => &[],
            CShell => &[],
            Cabal => &[],
            Cassius => &[],
            Ceylon => &[],
            Clojure => &[],
            ClojureC => &[],
            ClojureScript => &[],
            Cobol => &[],
            CodeQL => &[],
            CoffeeScript => &[],
            Cogent => &[],
            ColdFusion => &[],
            ColdFusionScript => &[],
            Coq => &[],
            Cpp => &[],
            CppHeader => &[],
            Crystal => &[],
            Css => &[],
            D => &[("/+","+/",),],
            Daml => &[],
            Dart => &[],
            DeviceTree => &[],
            Dhall => &[],
            Dockerfile => &[],
            DotNetResource => &[],
            DreamMaker => &[],
            Dust => &[],
            Edn => &[],
            Elisp => &[],
            Elixir => &[],
            Elm => &[],
            Elvish => &[],
            EmacsDevEnv => &[],
            Emojicode => &[],
            Erlang => &[],
            FEN => &[],
            FSharp => &[],
            Fish => &[],
            FlatBuffers => &[],
            Forth => &[],
            FortranLegacy => &[],
            FortranModern => &[],
            FreeMarker => &[],
            Fstar => &[],
            Futhark => &[],
            GDB => &[],
            GdScript => &[],
            Gherkin => &[],
            Gleam => &[],
            Glsl => &[],
            Go => &[],
            Gohtml => &[],
            Graphql => &[],
            Groovy => &[],
            Gwion => &[],
            Hamlet => &[],
            Handlebars => &[],
            Happy => &[],
            Haskell => &[],
            Haxe => &[],
            Hcl => &[],
            Headache => &[],
            Hex => &[],
            Hlsl => &[],
            HolyC => &[],
            Html => &[],
            Idris => &[],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &[],
            Jai => &[],
            Java => &[],
            JavaScript => &[],
            Json => &[],
            Jsonnet => &[],
            Jsx => &[],
            Julia => &[],
            Julius => &[],
            Jupyter => &[],
            K => &[],
            KakouneScript => &[],
            Kotlin => &[],
            LLVM => &[],
            Lean => &[],
            Less => &[],
            LinkerScript => &[],
            Liquid => &[],
            Lisp => &[],
            LiveScript => &[],
            Logtalk => &[],
            Lua => &[],
            Lucius => &[],
            Madlang => &[],
            Makefile => &[],
            Markdown => &[],
            Meson => &[],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &[],
            MsBuild => &[],
            Mustache => &[],
            Nim => &[],
            Nix => &[],
            NotQuitePerl => &[],
            OCaml => &[],
            ObjectiveC => &[],
            ObjectiveCpp => &[],
            Odin => &[],
            OpenType => &[],
            Org => &[],
            Oz => &[],
            PSL => &[],
            Pan => &[],
            Pascal => &[],
            Perl => &[],
            Perl6 => &[],
            Pest => &[],
            Php => &[],
            Polly => &[],
            Pony => &[],
            PostCss => &[],
            PowerShell => &[],
            Processing => &[],
            Prolog => &[],
            Protobuf => &[],
            Pug => &[],
            PureScript => &[],
            Python => &[],
            Q => &[],
            Qcl => &[],
            Qml => &[],
            R => &[],
            RON => &[],
            RPMSpecfile => &[],
            Racket => &[],
            Rakefile => &[],
            Razor => &[],
            ReStructuredText => &[],
            Renpy => &[],
            Ruby => &[],
            RubyHtml => &[],
            Rust => &[],
            SRecode => &[],
            Sass => &[],
            Scala => &[],
            Scheme => &[],
            Scons => &[],
            Sh => &[],
            Sml => &[],
            Solidity => &[],
            SpecmanE => &[],
            Spice => &[],
            Sql => &[],
            Stan => &[],
            Stratego => &[],
            Stylus => &[],
            Svelte => &[],
            Svg => &[],
            Swift => &[],
            Swig => &[],
            SystemVerilog => &[],
            Tcl => &[],
            Tera => &[],
            Tex => &[],
            Text => &[],
            Thrift => &[],
            Toml => &[],
            Tsx => &[],
            Ttcn => &[],
            Twig => &[],
            TypeScript => &[],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &[],
            UnrealShader => &[],
            UnrealShaderHeader => &[],
            UrWeb => &[],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &[],
            Velocity => &[],
            Verilog => &[],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &[],
            VisualBasic => &[],
            VisualStudioProject => &[],
            VisualStudioSolution => &[],
            Vue => &[],
            WebAssembly => &[],
            Wolfram => &[],
            XSL => &[],
            Xaml => &[],
            XcodeConfig => &[],
            Xml => &[],
            Xtend => &[],
            Yaml => &[],
            Zig => &[],
            Zsh => &[],
            
        }
    }

    /// Returns the quotes of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::C;
    /// assert_eq!(lang.quotes(), &[("\"", "\"")]);
    /// ```
    pub fn quotes(self) -> &'static [(&'static str, &'static str)] {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &[("\"","\"",),],
            Ada => &[],
            Agda => &[],
            Alex => &[],
            Alloy => &[],
            Arduino => &[("\"","\"",),],
            AsciiDoc => &[],
            Asn1 => &[("\"","\"",),("'","'",),],
            Asp => &[],
            AspNet => &[],
            Assembly => &[("\"","\"",),("'","'",),],
            AssemblyGAS => &[("\"","\"",),],
            AutoHotKey => &[],
            Autoconf => &[],
            Automake => &[],
            Bash => &[("\"","\"",),("'","'",),],
            Batch => &[],
            Bean => &[("\"","\"",),],
            BrightScript => &[("\"","\"",),],
            C => &[("\"","\"",),],
            CHeader => &[("\"","\"",),],
            CMake => &[("\"","\"",),],
            CSharp => &[("\"","\"",),],
            CShell => &[],
            Cabal => &[],
            Cassius => &[("\"","\"",),("'","'",),],
            Ceylon => &[("\"\"\"","\"\"\"",),("\"","\"",),],
            Clojure => &[("\"","\"",),],
            ClojureC => &[("\"","\"",),],
            ClojureScript => &[("\"","\"",),],
            Cobol => &[],
            CodeQL => &[("\"","\"",),],
            CoffeeScript => &[("\"","\"",),("'","'",),],
            Cogent => &[],
            ColdFusion => &[("\"","\"",),("'","'",),],
            ColdFusionScript => &[("\"","\"",),],
            Coq => &[("\"","\"",),],
            Cpp => &[("\"","\"",),],
            CppHeader => &[("\"","\"",),],
            Crystal => &[("\"","\"",),("'","'",),],
            Css => &[("\"","\"",),("'","'",),],
            D => &[("\"","\"",),("'","'",),],
            Daml => &[],
            Dart => &[("\"\"\"","\"\"\"",),("'''","'''",),("\"","\"",),("'","'",),],
            DeviceTree => &[("\"","\"",),],
            Dhall => &[("\"","\"",),("''","''",),],
            Dockerfile => &[("\"","\"",),("'","'",),],
            DotNetResource => &[("\"","\"",),],
            DreamMaker => &[("{\"","\"}",),("\"","\"",),("'","'",),],
            Dust => &[],
            Edn => &[],
            Elisp => &[],
            Elixir => &[("\"\"\"","\"\"\"",),("'''","'''",),("\"","\"",),("'","'",),],
            Elm => &[],
            Elvish => &[("\"","\"",),("'","'",),],
            EmacsDevEnv => &[],
            Emojicode => &[("❌🔤","❌🔤",),],
            Erlang => &[],
            FEN => &[],
            FSharp => &[("\"","\"",),],
            Fish => &[("\"","\"",),("'","'",),],
            FlatBuffers => &[("\"","\"",),],
            Forth => &[],
            FortranLegacy => &[("\"","\"",),("'","'",),],
            FortranModern => &[("\"","\"",),],
            FreeMarker => &[],
            Fstar => &[("\"","\"",),],
            Futhark => &[],
            GDB => &[],
            GdScript => &[("\"\"\"","\"\"\"",),("\"","\"",),("'","'",),],
            Gherkin => &[],
            Gleam => &[("\"","\"",),],
            Glsl => &[("\"","\"",),],
            Go => &[("\"","\"",),],
            Gohtml => &[("\"","\"",),("'","'",),],
            Graphql => &[("\"\"\"","\"\"\"",),("\"","\"",),],
            Groovy => &[("\"","\"",),],
            Gwion => &[("\"","\"",),],
            Hamlet => &[("\"","\"",),("'","'",),],
            Handlebars => &[("\"","\"",),("'","'",),],
            Happy => &[],
            Haskell => &[],
            Haxe => &[("\"","\"",),("'","'",),],
            Hcl => &[("\"","\"",),],
            Headache => &[("\"","\"",),],
            Hex => &[],
            Hlsl => &[("\"","\"",),],
            HolyC => &[("\"","\"",),],
            Html => &[("\"","\"",),("'","'",),],
            Idris => &[("\"\"\"","\"\"\"",),("\"","\"",),],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &[("''","''",),],
            Jai => &[("\"","\"",),],
            Java => &[("\"","\"",),],
            JavaScript => &[("\"","\"",),("'","'",),("`","`",),],
            Json => &[],
            Jsonnet => &[("\"","\"",),("'","'",),],
            Jsx => &[("\"","\"",),("'","'",),("`","`",),],
            Julia => &[("\"\"\"","\"\"\"",),("\"","\"",),],
            Julius => &[("\"","\"",),("'","'",),("`","`",),],
            Jupyter => &[],
            K => &[("\"","\"",),],
            KakouneScript => &[("\"","\"",),("'","'",),],
            Kotlin => &[("\"\"\"","\"\"\"",),("\"","\"",),],
            LLVM => &[("\"","\"",),("'","'",),],
            Lean => &[],
            Less => &[("\"","\"",),("'","'",),],
            LinkerScript => &[("\"","\"",),],
            Liquid => &[("\"","\"",),("'","'",),],
            Lisp => &[],
            LiveScript => &[("\"","\"",),("'","'",),],
            Logtalk => &[("\"","\"",),],
            Lua => &[("\"","\"",),("'","'",),],
            Lucius => &[("\"","\"",),("'","'",),],
            Madlang => &[],
            Makefile => &[],
            Markdown => &[],
            Meson => &[("'''","'''",),("'","'",),],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &[("\"","\"",),("'","'",),],
            MsBuild => &[("\"","\"",),("'","'",),],
            Mustache => &[("\"","\"",),("'","'",),],
            Nim => &[("\"\"\"","\"\"\"",),("\"","\"",),],
            Nix => &[("\"","\"",),],
            NotQuitePerl => &[("\"","\"",),("'","'",),],
            OCaml => &[("\"","\"",),],
            ObjectiveC => &[("\"","\"",),],
            ObjectiveCpp => &[("\"","\"",),],
            Odin => &[("\"","\"",),("'","'",),],
            OpenType => &[],
            Org => &[],
            Oz => &[("\"","\"",),],
            PSL => &[("\"","\"",),],
            Pan => &[("\"","\"",),("'","'",),],
            Pascal => &[("'","'",),],
            Perl => &[("\"","\"",),("'","'",),],
            Perl6 => &[("\"","\"",),("'","'",),],
            Pest => &[("\"","\"",),("'","'",),],
            Php => &[("\"","\"",),("'","'",),],
            Polly => &[("\"","\"",),("'","'",),],
            Pony => &[("\"","\"",),],
            PostCss => &[("\"","\"",),("'","'",),],
            PowerShell => &[("\"@","@\"",),("\"","\"",),("@'","'@",),("'","'",),],
            Processing => &[("\"","\"",),],
            Prolog => &[("\"","\"",),],
            Protobuf => &[],
            Pug => &[("#{\"","\"}",),("#{'","'}",),("#{`","`}",),],
            PureScript => &[],
            Python => &[("\"","\"",),("'","'",),],
            Q => &[("\"","\"",),],
            Qcl => &[("\"","\"",),],
            Qml => &[("\"","\"",),("'","'",),],
            R => &[],
            RON => &[("\"","\"",),],
            RPMSpecfile => &[],
            Racket => &[],
            Rakefile => &[("\"","\"",),("'","'",),],
            Razor => &[],
            ReStructuredText => &[],
            Renpy => &[("\"","\"",),("'","'",),("`","`",),],
            Ruby => &[("\"","\"",),("'","'",),],
            RubyHtml => &[("\"","\"",),("'","'",),],
            Rust => &[("#\"","\"#",),("\"","\"",),],
            SRecode => &[],
            Sass => &[("\"","\"",),("'","'",),],
            Scala => &[("\"","\"",),],
            Scheme => &[],
            Scons => &[("\"\"\"","\"\"\"",),("'''","'''",),("\"","\"",),("'","'",),],
            Sh => &[("\"","\"",),("'","'",),],
            Sml => &[("\"","\"",),],
            Solidity => &[("\"","\"",),],
            SpecmanE => &[],
            Spice => &[],
            Sql => &[("'","'",),],
            Stan => &[("\"","\"",),],
            Stratego => &[("\"","\"",),("$[","]",),("$<",">",),("${","}",),],
            Stylus => &[("\"","\"",),("'","'",),],
            Svelte => &[("\"","\"",),("'","'",),],
            Svg => &[("\"","\"",),("'","'",),],
            Swift => &[("\"","\"",),],
            Swig => &[("\"","\"",),],
            SystemVerilog => &[("\"","\"",),],
            Tcl => &[("\"","\"",),("'","'",),],
            Tera => &[("\"","\"",),("'","'",),],
            Tex => &[],
            Text => &[],
            Thrift => &[("\"","\"",),("'","'",),],
            Toml => &[("\"\"\"","\"\"\"",),("'''","'''",),("\"","\"",),("'","'",),],
            Tsx => &[("\"","\"",),("'","'",),("`","`",),],
            Ttcn => &[("\"","\"",),],
            Twig => &[("\"","\"",),("'","'",),],
            TypeScript => &[("\"","\"",),("'","'",),("`","`",),],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &[("\"","\"",),],
            UnrealShader => &[("\"","\"",),],
            UnrealShaderHeader => &[("\"","\"",),],
            UrWeb => &[("\"","\"",),],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &[("\"","\"",),],
            Velocity => &[("\"","\"",),("'","'",),],
            Verilog => &[("\"","\"",),],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &[("\"","\"",),("'","'",),],
            VisualBasic => &[("\"","\"",),],
            VisualStudioProject => &[("\"","\"",),("'","'",),],
            VisualStudioSolution => &[],
            Vue => &[("\"","\"",),("'","'",),("`","`",),],
            WebAssembly => &[("\"","\"",),("'","'",),],
            Wolfram => &[("\"","\"",),],
            XSL => &[("\"","\"",),("'","'",),],
            Xaml => &[("\"","\"",),("'","'",),],
            XcodeConfig => &[("\"","\"",),("'","'",),],
            Xml => &[("\"","\"",),("'","'",),],
            Xtend => &[("'''","'''",),("\"","\"",),("'","'",),],
            Yaml => &[("\"","\"",),("'","'",),],
            Zig => &[("\"","\"",),],
            Zsh => &[("\"","\"",),("'","'",),],
            
        }
    }

    /// Returns the verbatim quotes of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::CSharp;
    /// assert_eq!(lang.verbatim_quotes(), &[("@\"", "\"")]);
    /// ```
    pub fn verbatim_quotes(self) -> &'static [(&'static str, &'static str)] {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &[],
            Ada => &[],
            Agda => &[],
            Alex => &[],
            Alloy => &[],
            Arduino => &[],
            AsciiDoc => &[],
            Asn1 => &[],
            Asp => &[],
            AspNet => &[],
            Assembly => &[],
            AssemblyGAS => &[],
            AutoHotKey => &[],
            Autoconf => &[],
            Automake => &[],
            Bash => &[],
            Batch => &[],
            Bean => &[],
            BrightScript => &[],
            C => &[],
            CHeader => &[],
            CMake => &[],
            CSharp => &[("@\"","\"",),],
            CShell => &[],
            Cabal => &[],
            Cassius => &[],
            Ceylon => &[],
            Clojure => &[],
            ClojureC => &[],
            ClojureScript => &[],
            Cobol => &[],
            CodeQL => &[],
            CoffeeScript => &[],
            Cogent => &[],
            ColdFusion => &[],
            ColdFusionScript => &[],
            Coq => &[],
            Cpp => &[("R\"(",")\"",),],
            CppHeader => &[],
            Crystal => &[],
            Css => &[],
            D => &[],
            Daml => &[],
            Dart => &[],
            DeviceTree => &[],
            Dhall => &[],
            Dockerfile => &[],
            DotNetResource => &[],
            DreamMaker => &[],
            Dust => &[],
            Edn => &[],
            Elisp => &[],
            Elixir => &[],
            Elm => &[],
            Elvish => &[],
            EmacsDevEnv => &[],
            Emojicode => &[],
            Erlang => &[],
            FEN => &[],
            FSharp => &[("@\"","\"",),],
            Fish => &[],
            FlatBuffers => &[],
            Forth => &[],
            FortranLegacy => &[],
            FortranModern => &[],
            FreeMarker => &[],
            Fstar => &[],
            Futhark => &[],
            GDB => &[],
            GdScript => &[],
            Gherkin => &[],
            Gleam => &[],
            Glsl => &[],
            Go => &[],
            Gohtml => &[],
            Graphql => &[],
            Groovy => &[],
            Gwion => &[],
            Hamlet => &[],
            Handlebars => &[],
            Happy => &[],
            Haskell => &[],
            Haxe => &[],
            Hcl => &[],
            Headache => &[],
            Hex => &[],
            Hlsl => &[],
            HolyC => &[],
            Html => &[],
            Idris => &[],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &[],
            Jai => &[],
            Java => &[],
            JavaScript => &[],
            Json => &[],
            Jsonnet => &[],
            Jsx => &[],
            Julia => &[],
            Julius => &[],
            Jupyter => &[],
            K => &[],
            KakouneScript => &[],
            Kotlin => &[],
            LLVM => &[],
            Lean => &[],
            Less => &[],
            LinkerScript => &[],
            Liquid => &[],
            Lisp => &[],
            LiveScript => &[],
            Logtalk => &[],
            Lua => &[],
            Lucius => &[],
            Madlang => &[],
            Makefile => &[],
            Markdown => &[],
            Meson => &[],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &[],
            MsBuild => &[],
            Mustache => &[],
            Nim => &[],
            Nix => &[],
            NotQuitePerl => &[],
            OCaml => &[],
            ObjectiveC => &[],
            ObjectiveCpp => &[],
            Odin => &[],
            OpenType => &[],
            Org => &[],
            Oz => &[],
            PSL => &[],
            Pan => &[],
            Pascal => &[],
            Perl => &[],
            Perl6 => &[],
            Pest => &[],
            Php => &[],
            Polly => &[],
            Pony => &[],
            PostCss => &[],
            PowerShell => &[],
            Processing => &[],
            Prolog => &[],
            Protobuf => &[],
            Pug => &[],
            PureScript => &[],
            Python => &[],
            Q => &[],
            Qcl => &[],
            Qml => &[],
            R => &[],
            RON => &[],
            RPMSpecfile => &[],
            Racket => &[],
            Rakefile => &[],
            Razor => &[],
            ReStructuredText => &[],
            Renpy => &[],
            Ruby => &[],
            RubyHtml => &[],
            Rust => &[("r##\"","\"##",),("r#\"","\"#",),],
            SRecode => &[],
            Sass => &[],
            Scala => &[],
            Scheme => &[],
            Scons => &[],
            Sh => &[],
            Sml => &[],
            Solidity => &[],
            SpecmanE => &[],
            Spice => &[],
            Sql => &[],
            Stan => &[],
            Stratego => &[],
            Stylus => &[],
            Svelte => &[],
            Svg => &[],
            Swift => &[],
            Swig => &[],
            SystemVerilog => &[],
            Tcl => &[],
            Tera => &[],
            Tex => &[],
            Text => &[],
            Thrift => &[],
            Toml => &[],
            Tsx => &[],
            Ttcn => &[],
            Twig => &[],
            TypeScript => &[],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &[],
            UnrealShader => &[],
            UnrealShaderHeader => &[],
            UrWeb => &[],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &[],
            Velocity => &[],
            Verilog => &[],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &[],
            VisualBasic => &[],
            VisualStudioProject => &[],
            VisualStudioSolution => &[],
            Vue => &[],
            WebAssembly => &[],
            Wolfram => &[],
            XSL => &[],
            Xaml => &[],
            XcodeConfig => &[],
            Xml => &[],
            Xtend => &[],
            Yaml => &[],
            Zig => &[],
            Zsh => &[],
            
        }
    }

    /// Returns the doc quotes of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Python;
    /// assert_eq!(lang.doc_quotes(), &[("\"\"\"", "\"\"\""), ("'''", "'''")]);
    /// ```
    pub fn doc_quotes(self) -> &'static [(&'static str, &'static str)] {
        match self {
            ABNF => &[
                    
                ],Abap => &[
                    
                ],ActionScript => &[
                    
                ],Ada => &[
                    
                ],Agda => &[
                    
                ],Alex => &[
                    
                ],Alloy => &[
                    
                ],Arduino => &[
                    
                ],AsciiDoc => &[
                    
                ],Asn1 => &[
                    
                ],Asp => &[
                    
                ],AspNet => &[
                    
                ],Assembly => &[
                    
                ],AssemblyGAS => &[
                    
                ],AutoHotKey => &[
                    
                ],Autoconf => &[
                    
                ],Automake => &[
                    
                ],Bash => &[
                    
                ],Batch => &[
                    
                ],Bean => &[
                    
                ],BrightScript => &[
                    
                ],C => &[
                    
                ],CHeader => &[
                    
                ],CMake => &[
                    
                ],CSharp => &[
                    
                ],CShell => &[
                    
                ],Cabal => &[
                    
                ],Cassius => &[
                    
                ],Ceylon => &[
                    
                ],Clojure => &[
                    
                ],ClojureC => &[
                    
                ],ClojureScript => &[
                    
                ],Cobol => &[
                    
                ],CodeQL => &[
                    
                ],CoffeeScript => &[
                    
                ],Cogent => &[
                    
                ],ColdFusion => &[
                    
                ],ColdFusionScript => &[
                    
                ],Coq => &[
                    
                ],Cpp => &[
                    
                ],CppHeader => &[
                    
                ],Crystal => &[
                    
                ],Css => &[
                    
                ],D => &[
                    
                ],Daml => &[
                    
                ],Dart => &[
                    
                ],DeviceTree => &[
                    
                ],Dhall => &[
                    
                ],Dockerfile => &[
                    
                ],DotNetResource => &[
                    
                ],DreamMaker => &[
                    
                ],Dust => &[
                    
                ],Edn => &[
                    
                ],Elisp => &[
                    
                ],Elixir => &[
                    
                ],Elm => &[
                    
                ],Elvish => &[
                    
                ],EmacsDevEnv => &[
                    
                ],Emojicode => &[
                    
                ],Erlang => &[
                    
                ],FEN => &[
                    
                ],FSharp => &[
                    
                ],Fish => &[
                    
                ],FlatBuffers => &[
                    
                ],Forth => &[
                    
                ],FortranLegacy => &[
                    
                ],FortranModern => &[
                    
                ],FreeMarker => &[
                    
                ],Fstar => &[
                    
                ],Futhark => &[
                    
                ],GDB => &[
                    
                ],GdScript => &[
                    
                ],Gherkin => &[
                    
                ],Gleam => &[
                    
                ],Glsl => &[
                    
                ],Go => &[
                    
                ],Gohtml => &[
                    
                ],Graphql => &[
                    
                ],Groovy => &[
                    
                ],Gwion => &[
                    
                ],Hamlet => &[
                    
                ],Handlebars => &[
                    
                ],Happy => &[
                    
                ],Haskell => &[
                    
                ],Haxe => &[
                    
                ],Hcl => &[
                    
                ],Headache => &[
                    
                ],Hex => &[
                    
                ],Hlsl => &[
                    
                ],HolyC => &[
                    
                ],Html => &[
                    
                ],Idris => &[
                    
                ],Ini => &[
                    
                ],IntelHex => &[
                    
                ],Isabelle => &[
                    
                ],Jai => &[
                    
                ],Java => &[
                    
                ],JavaScript => &[
                    
                ],Json => &[
                    
                ],Jsonnet => &[
                    
                ],Jsx => &[
                    
                ],Julia => &[
                    
                ],Julius => &[
                    
                ],Jupyter => &[
                    
                ],K => &[
                    
                ],KakouneScript => &[
                    
                ],Kotlin => &[
                    
                ],LLVM => &[
                    
                ],Lean => &[
                    
                ],Less => &[
                    
                ],LinkerScript => &[
                    
                ],Liquid => &[
                    
                ],Lisp => &[
                    
                ],LiveScript => &[
                    
                ],Logtalk => &[
                    
                ],Lua => &[
                    
                ],Lucius => &[
                    
                ],Madlang => &[
                    
                ],Makefile => &[
                    
                ],Markdown => &[
                    
                ],Meson => &[
                    
                ],Mint => &[
                    
                ],ModuleDef => &[
                    
                ],MoonScript => &[
                    
                ],MsBuild => &[
                    
                ],Mustache => &[
                    
                ],Nim => &[
                    
                ],Nix => &[
                    
                ],NotQuitePerl => &[
                    
                ],OCaml => &[
                    
                ],ObjectiveC => &[
                    
                ],ObjectiveCpp => &[
                    
                ],Odin => &[
                    
                ],OpenType => &[
                    
                ],Org => &[
                    
                ],Oz => &[
                    
                ],PSL => &[
                    
                ],Pan => &[
                    
                ],Pascal => &[
                    
                ],Perl => &[
                    
                ],Perl6 => &[
                    
                ],Pest => &[
                    
                ],Php => &[
                    
                ],Polly => &[
                    
                ],Pony => &[
                    ("\"\"\"","\"\"\"",),
                ],PostCss => &[
                    
                ],PowerShell => &[
                    
                ],Processing => &[
                    
                ],Prolog => &[
                    
                ],Protobuf => &[
                    
                ],Pug => &[
                    
                ],PureScript => &[
                    
                ],Python => &[
                    ("\"\"\"","\"\"\"",),("'''","'''",),
                ],Q => &[
                    
                ],Qcl => &[
                    
                ],Qml => &[
                    
                ],R => &[
                    
                ],RON => &[
                    
                ],RPMSpecfile => &[
                    
                ],Racket => &[
                    
                ],Rakefile => &[
                    
                ],Razor => &[
                    
                ],ReStructuredText => &[
                    
                ],Renpy => &[
                    
                ],Ruby => &[
                    
                ],RubyHtml => &[
                    
                ],Rust => &[
                    
                ],SRecode => &[
                    
                ],Sass => &[
                    
                ],Scala => &[
                    
                ],Scheme => &[
                    
                ],Scons => &[
                    
                ],Sh => &[
                    
                ],Sml => &[
                    
                ],Solidity => &[
                    
                ],SpecmanE => &[
                    
                ],Spice => &[
                    
                ],Sql => &[
                    
                ],Stan => &[
                    
                ],Stratego => &[
                    
                ],Stylus => &[
                    
                ],Svelte => &[
                    
                ],Svg => &[
                    
                ],Swift => &[
                    
                ],Swig => &[
                    
                ],SystemVerilog => &[
                    
                ],Tcl => &[
                    
                ],Tera => &[
                    
                ],Tex => &[
                    
                ],Text => &[
                    
                ],Thrift => &[
                    
                ],Toml => &[
                    
                ],Tsx => &[
                    
                ],Ttcn => &[
                    
                ],Twig => &[
                    
                ],TypeScript => &[
                    
                ],UnrealDeveloperMarkdown => &[
                    
                ],UnrealPlugin => &[
                    
                ],UnrealProject => &[
                    
                ],UnrealScript => &[
                    
                ],UnrealShader => &[
                    
                ],UnrealShaderHeader => &[
                    
                ],UrWeb => &[
                    
                ],UrWebProject => &[
                    
                ],VB6 => &[
                    
                ],VBScript => &[
                    
                ],Vala => &[
                    
                ],Velocity => &[
                    
                ],Verilog => &[
                    
                ],VerilogArgsFile => &[
                    
                ],Vhdl => &[
                    
                ],VimScript => &[
                    
                ],VisualBasic => &[
                    
                ],VisualStudioProject => &[
                    
                ],VisualStudioSolution => &[
                    
                ],Vue => &[
                    
                ],WebAssembly => &[
                    
                ],Wolfram => &[
                    
                ],XSL => &[
                    
                ],Xaml => &[
                    
                ],XcodeConfig => &[
                    
                ],Xml => &[
                    
                ],Xtend => &[
                    
                ],Yaml => &[
                    
                ],Zig => &[
                    
                ],Zsh => &[
                    
                ],
        }
    }

    /// Returns the shebang of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Bash;
    /// assert_eq!(lang.shebangs(), &["#!/bin/bash"]);
    /// ```
    pub fn shebangs(self) -> &'static [&'static str] {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &[],
            Ada => &[],
            Agda => &[],
            Alex => &[],
            Alloy => &[],
            Arduino => &[],
            AsciiDoc => &[],
            Asn1 => &[],
            Asp => &[],
            AspNet => &[],
            Assembly => &[],
            AssemblyGAS => &[],
            AutoHotKey => &[],
            Autoconf => &[],
            Automake => &[],
            Bash => &["#!/bin/bash",],
            Batch => &[],
            Bean => &[],
            BrightScript => &[],
            C => &[],
            CHeader => &[],
            CMake => &[],
            CSharp => &[],
            CShell => &["#!/bin/csh",],
            Cabal => &[],
            Cassius => &[],
            Ceylon => &[],
            Clojure => &[],
            ClojureC => &[],
            ClojureScript => &[],
            Cobol => &[],
            CodeQL => &[],
            CoffeeScript => &[],
            Cogent => &[],
            ColdFusion => &[],
            ColdFusionScript => &[],
            Coq => &[],
            Cpp => &[],
            CppHeader => &[],
            Crystal => &["#!/usr/bin/crystal",],
            Css => &[],
            D => &[],
            Daml => &[],
            Dart => &[],
            DeviceTree => &[],
            Dhall => &[],
            Dockerfile => &[],
            DotNetResource => &[],
            DreamMaker => &[],
            Dust => &[],
            Edn => &[],
            Elisp => &[],
            Elixir => &[],
            Elm => &[],
            Elvish => &[],
            EmacsDevEnv => &[],
            Emojicode => &[],
            Erlang => &[],
            FEN => &[],
            FSharp => &[],
            Fish => &["#!/bin/fish",],
            FlatBuffers => &[],
            Forth => &[],
            FortranLegacy => &[],
            FortranModern => &[],
            FreeMarker => &[],
            Fstar => &[],
            Futhark => &[],
            GDB => &[],
            GdScript => &[],
            Gherkin => &[],
            Gleam => &[],
            Glsl => &[],
            Go => &[],
            Gohtml => &[],
            Graphql => &[],
            Groovy => &[],
            Gwion => &[],
            Hamlet => &[],
            Handlebars => &[],
            Happy => &[],
            Haskell => &[],
            Haxe => &[],
            Hcl => &[],
            Headache => &[],
            Hex => &[],
            Hlsl => &[],
            HolyC => &[],
            Html => &[],
            Idris => &[],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &[],
            Jai => &[],
            Java => &[],
            JavaScript => &[],
            Json => &[],
            Jsonnet => &[],
            Jsx => &[],
            Julia => &[],
            Julius => &[],
            Jupyter => &[],
            K => &[],
            KakouneScript => &[],
            Kotlin => &[],
            LLVM => &[],
            Lean => &[],
            Less => &[],
            LinkerScript => &[],
            Liquid => &[],
            Lisp => &[],
            LiveScript => &[],
            Logtalk => &[],
            Lua => &[],
            Lucius => &[],
            Madlang => &[],
            Makefile => &[],
            Markdown => &[],
            Meson => &[],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &[],
            MsBuild => &[],
            Mustache => &[],
            Nim => &[],
            Nix => &[],
            NotQuitePerl => &[],
            OCaml => &[],
            ObjectiveC => &[],
            ObjectiveCpp => &[],
            Odin => &[],
            OpenType => &[],
            Org => &[],
            Oz => &[],
            PSL => &[],
            Pan => &[],
            Pascal => &[],
            Perl => &["#!/usr/bin/perl",],
            Perl6 => &[],
            Pest => &[],
            Php => &[],
            Polly => &[],
            Pony => &[],
            PostCss => &[],
            PowerShell => &[],
            Processing => &[],
            Prolog => &[],
            Protobuf => &[],
            Pug => &[],
            PureScript => &[],
            Python => &[],
            Q => &[],
            Qcl => &[],
            Qml => &[],
            R => &[],
            RON => &[],
            RPMSpecfile => &[],
            Racket => &[],
            Rakefile => &[],
            Razor => &[],
            ReStructuredText => &[],
            Renpy => &[],
            Ruby => &[],
            RubyHtml => &[],
            Rust => &[],
            SRecode => &[],
            Sass => &[],
            Scala => &[],
            Scheme => &[],
            Scons => &[],
            Sh => &["#!/bin/sh",],
            Sml => &[],
            Solidity => &[],
            SpecmanE => &[],
            Spice => &[],
            Sql => &[],
            Stan => &[],
            Stratego => &[],
            Stylus => &[],
            Svelte => &[],
            Svg => &[],
            Swift => &[],
            Swig => &[],
            SystemVerilog => &[],
            Tcl => &[],
            Tera => &[],
            Tex => &[],
            Text => &[],
            Thrift => &[],
            Toml => &[],
            Tsx => &[],
            Ttcn => &[],
            Twig => &[],
            TypeScript => &[],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &[],
            UnrealShader => &[],
            UnrealShaderHeader => &[],
            UrWeb => &[],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &[],
            Velocity => &[],
            Verilog => &[],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &[],
            VisualBasic => &[],
            VisualStudioProject => &[],
            VisualStudioSolution => &[],
            Vue => &[],
            WebAssembly => &[],
            Wolfram => &[],
            XSL => &[],
            Xaml => &[],
            XcodeConfig => &[],
            Xml => &[],
            Xtend => &[],
            Yaml => &[],
            Zig => &[],
            Zsh => &["#!/bin/zsh",],
            
        }
    }

    pub(crate) fn any_multi_line_comments(self) -> &'static [(&'static str, &'static str)] {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &[("/*", "*/"),],
            Ada => &[],
            Agda => &[("{-", "-}"),],
            Alex => &[],
            Alloy => &[("/*", "*/"),],
            Arduino => &[("/*", "*/"),],
            AsciiDoc => &[("////", "////"),],
            Asn1 => &[("/*", "*/"),],
            Asp => &[],
            AspNet => &[("<!--", "-->"),("<%--", "-->"),],
            Assembly => &[],
            AssemblyGAS => &[("/*", "*/"),],
            AutoHotKey => &[("/*", "*/"),],
            Autoconf => &[],
            Automake => &[],
            Bash => &[],
            Batch => &[],
            Bean => &[],
            BrightScript => &[],
            C => &[("/*", "*/"),],
            CHeader => &[("/*", "*/"),],
            CMake => &[],
            CSharp => &[("/*", "*/"),],
            CShell => &[],
            Cabal => &[("{-", "-}"),],
            Cassius => &[("/*", "*/"),],
            Ceylon => &[("/*", "*/"),],
            Clojure => &[],
            ClojureC => &[],
            ClojureScript => &[],
            Cobol => &[],
            CodeQL => &[("/*", "*/"),],
            CoffeeScript => &[("###", "###"),],
            Cogent => &[],
            ColdFusion => &[("<!---", "--->"),],
            ColdFusionScript => &[("/*", "*/"),],
            Coq => &[("(*", "*)"),],
            Cpp => &[("/*", "*/"),],
            CppHeader => &[("/*", "*/"),],
            Crystal => &[],
            Css => &[("/*", "*/"),],
            D => &[("/*", "*/"),("/+", "+/"),],
            Daml => &[("{-", "-}"),],
            Dart => &[("/*", "*/"),],
            DeviceTree => &[("/*", "*/"),],
            Dhall => &[("{-", "-}"),],
            Dockerfile => &[],
            DotNetResource => &[("<!--", "-->"),],
            DreamMaker => &[("/*", "*/"),],
            Dust => &[("{!", "!}"),],
            Edn => &[],
            Elisp => &[],
            Elixir => &[],
            Elm => &[("{-", "-}"),],
            Elvish => &[],
            EmacsDevEnv => &[],
            Emojicode => &[("💭🔜", "🔚💭"),("📗", "📗"),("📘", "📘"),],
            Erlang => &[],
            FEN => &[],
            FSharp => &[("(*", "*)"),],
            Fish => &[],
            FlatBuffers => &[("/*", "*/"),],
            Forth => &[("( ", ")"),],
            FortranLegacy => &[],
            FortranModern => &[],
            FreeMarker => &[("<#--", "-->"),],
            Fstar => &[("(*", "*)"),],
            Futhark => &[],
            GDB => &[],
            GdScript => &[],
            Gherkin => &[],
            Gleam => &[],
            Glsl => &[("/*", "*/"),],
            Go => &[("/*", "*/"),],
            Gohtml => &[("<!--", "-->"),("{{/*", "*/}}"),],
            Graphql => &[],
            Groovy => &[("/*", "*/"),],
            Gwion => &[],
            Hamlet => &[("<!--", "-->"),],
            Handlebars => &[("<!--", "-->"),("{{!", "}}"),],
            Happy => &[],
            Haskell => &[("{-", "-}"),],
            Haxe => &[("/*", "*/"),],
            Hcl => &[("/*", "*/"),],
            Headache => &[("/*", "*/"),],
            Hex => &[],
            Hlsl => &[("/*", "*/"),],
            HolyC => &[("/*", "*/"),],
            Html => &[("<!--", "-->"),],
            Idris => &[("{-", "-}"),],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &[("{*", "*}"),("(*", "*)"),("‹", "›"),("\\<open>", "\\<close>"),],
            Jai => &[("/*", "*/"),],
            Java => &[("/*", "*/"),],
            JavaScript => &[("/*", "*/"),],
            Json => &[],
            Jsonnet => &[("/*", "*/"),],
            Jsx => &[("/*", "*/"),],
            Julia => &[("#=", "=#"),],
            Julius => &[("/*", "*/"),],
            Jupyter => &[],
            K => &[],
            KakouneScript => &[],
            Kotlin => &[("/*", "*/"),],
            LLVM => &[],
            Lean => &[("/-", "-/"),],
            Less => &[("/*", "*/"),],
            LinkerScript => &[("/*", "*/"),],
            Liquid => &[("<!--", "-->"),("{% comment %}", "{% endcomment %}"),],
            Lisp => &[("#|", "|#"),],
            LiveScript => &[("/*", "*/"),],
            Logtalk => &[("/*", "*/"),],
            Lua => &[("--[[", "]]"),],
            Lucius => &[("/*", "*/"),],
            Madlang => &[("{#", "#}"),],
            Makefile => &[],
            Markdown => &[],
            Meson => &[],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &[],
            MsBuild => &[("<!--", "-->"),],
            Mustache => &[("{{!", "}}"),],
            Nim => &[],
            Nix => &[("/*", "*/"),],
            NotQuitePerl => &[("=begin", "=end"),],
            OCaml => &[("(*", "*)"),],
            ObjectiveC => &[("/*", "*/"),],
            ObjectiveCpp => &[("/*", "*/"),],
            Odin => &[("/*", "*/"),],
            OpenType => &[],
            Org => &[],
            Oz => &[("/*", "*/"),],
            PSL => &[("/*", "*/"),],
            Pan => &[],
            Pascal => &[("{", "}"),("(*", "*)"),],
            Perl => &[("=pod", "=cut"),],
            Perl6 => &[("=begin", "=end"),],
            Pest => &[],
            Php => &[("/*", "*/"),],
            Polly => &[("<!--", "-->"),],
            Pony => &[("/*", "*/"),],
            PostCss => &[("/*", "*/"),],
            PowerShell => &[("<#", "#>"),],
            Processing => &[("/*", "*/"),],
            Prolog => &[("/*", "*/"),],
            Protobuf => &[],
            Pug => &[],
            PureScript => &[("{-", "-}"),],
            Python => &[],
            Q => &[],
            Qcl => &[("/*", "*/"),],
            Qml => &[("/*", "*/"),],
            R => &[],
            RON => &[("/*", "*/"),],
            RPMSpecfile => &[],
            Racket => &[("#|", "|#"),],
            Rakefile => &[("=begin", "=end"),],
            Razor => &[("<!--", "-->"),("@*", "*@"),],
            ReStructuredText => &[],
            Renpy => &[],
            Ruby => &[("=begin", "=end"),],
            RubyHtml => &[("<!--", "-->"),],
            Rust => &[("/*", "*/"),],
            SRecode => &[],
            Sass => &[("/*", "*/"),],
            Scala => &[("/*", "*/"),],
            Scheme => &[("#|", "|#"),],
            Scons => &[],
            Sh => &[],
            Sml => &[("(*", "*)"),],
            Solidity => &[("/*", "*/"),],
            SpecmanE => &[("'>", "<'"),],
            Spice => &[],
            Sql => &[("/*", "*/"),],
            Stan => &[("/*", "*/"),],
            Stratego => &[("/*", "*/"),],
            Stylus => &[("/*", "*/"),],
            Svelte => &[("<!--", "-->"),],
            Svg => &[("<!--", "-->"),],
            Swift => &[("/*", "*/"),],
            Swig => &[("/*", "*/"),],
            SystemVerilog => &[("/*", "*/"),],
            Tcl => &[],
            Tera => &[("<!--", "-->"),("{#", "#}"),],
            Tex => &[],
            Text => &[],
            Thrift => &[("/*", "*/"),],
            Toml => &[],
            Tsx => &[("/*", "*/"),],
            Ttcn => &[("/*", "*/"),],
            Twig => &[("<!--", "-->"),("{#", "#}"),],
            TypeScript => &[("/*", "*/"),],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &[("/*", "*/"),],
            UnrealShader => &[("/*", "*/"),],
            UnrealShaderHeader => &[("/*", "*/"),],
            UrWeb => &[("(*", "*)"),],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &[("/*", "*/"),],
            Velocity => &[("#*", "*#"),],
            Verilog => &[("/*", "*/"),],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &[],
            VisualBasic => &[],
            VisualStudioProject => &[("<!--", "-->"),],
            VisualStudioSolution => &[],
            Vue => &[("<!--", "-->"),("/*", "*/"),],
            WebAssembly => &[],
            Wolfram => &[("(*", "*)"),],
            XSL => &[("<!--", "-->"),],
            Xaml => &[("<!--", "-->"),],
            XcodeConfig => &[],
            Xml => &[("<!--", "-->"),],
            Xtend => &[("/*", "*/"),],
            Yaml => &[],
            Zig => &[],
            Zsh => &[],
            
        }
    }

    pub(crate) fn any_comments(self) -> &'static [&'static str] {
        match self {
            ABNF => &[";",],
            Abap => &["*","\"",],
            ActionScript => &["/*",
                        "*/","//",],
            Ada => &["--",],
            Agda => &["{-",
                        "-}","--",],
            Alex => &[],
            Alloy => &["/*",
                        "*/","--","//",],
            Arduino => &["/*",
                        "*/","//",],
            AsciiDoc => &["////",
                        "////","//",],
            Asn1 => &["/*",
                        "*/","--",],
            Asp => &["'","REM",],
            AspNet => &["<!--",
                        "-->","<%--",
                        "-->",],
            Assembly => &[";",],
            AssemblyGAS => &["/*",
                        "*/","//",],
            AutoHotKey => &["/*",
                        "*/",";",],
            Autoconf => &["#","dnl",],
            Automake => &["#",],
            Bash => &["#",],
            Batch => &["REM","::",],
            Bean => &[";",],
            BrightScript => &["'","REM",],
            C => &["/*",
                        "*/","//",],
            CHeader => &["/*",
                        "*/","//",],
            CMake => &["#",],
            CSharp => &["/*",
                        "*/","//",],
            CShell => &["#",],
            Cabal => &["{-",
                        "-}","--",],
            Cassius => &["/*",
                        "*/","//",],
            Ceylon => &["/*",
                        "*/","//",],
            Clojure => &[";",],
            ClojureC => &[";",],
            ClojureScript => &[";",],
            Cobol => &["*",],
            CodeQL => &["/*",
                        "*/","//",],
            CoffeeScript => &["###",
                        "###","#",],
            Cogent => &["--",],
            ColdFusion => &["<!---",
                        "--->",],
            ColdFusionScript => &["/*",
                        "*/","//",],
            Coq => &["(*",
                        "*)",],
            Cpp => &["/*",
                        "*/","//",],
            CppHeader => &["/*",
                        "*/","//",],
            Crystal => &["#",],
            Css => &["/*",
                        "*/","//",],
            D => &["/*",
                        "*/","/+",
                        "+/","//",],
            Daml => &["{-",
                        "-}","-- ",],
            Dart => &["/*",
                        "*/","//",],
            DeviceTree => &["/*",
                        "*/","//",],
            Dhall => &["{-",
                        "-}","--",],
            Dockerfile => &["#",],
            DotNetResource => &["<!--",
                        "-->",],
            DreamMaker => &["/*",
                        "*/","//",],
            Dust => &["{!",
                        "!}",],
            Edn => &[";",],
            Elisp => &[";",],
            Elixir => &["#",],
            Elm => &["{-",
                        "-}","--",],
            Elvish => &["#",],
            EmacsDevEnv => &[";",],
            Emojicode => &["💭🔜",
                        "🔚💭","📗",
                        "📗","📘",
                        "📘","💭",],
            Erlang => &["%",],
            FEN => &[],
            FSharp => &["(*",
                        "*)","//",],
            Fish => &["#",],
            FlatBuffers => &["/*",
                        "*/","//",],
            Forth => &["( ",
                        ")","\\",],
            FortranLegacy => &["c","C","!","*",],
            FortranModern => &["!",],
            FreeMarker => &["<#--",
                        "-->",],
            Fstar => &["(*",
                        "*)","//",],
            Futhark => &["--",],
            GDB => &["#",],
            GdScript => &["#",],
            Gherkin => &["#",],
            Gleam => &["//","///","////",],
            Glsl => &["/*",
                        "*/","//",],
            Go => &["/*",
                        "*/","//",],
            Gohtml => &["<!--",
                        "-->","{{/*",
                        "*/}}",],
            Graphql => &["#",],
            Groovy => &["/*",
                        "*/","//",],
            Gwion => &["#!",],
            Hamlet => &["<!--",
                        "-->",],
            Handlebars => &["<!--",
                        "-->","{{!",
                        "}}",],
            Happy => &[],
            Haskell => &["{-",
                        "-}","--",],
            Haxe => &["/*",
                        "*/","//",],
            Hcl => &["/*",
                        "*/","#","//",],
            Headache => &["/*",
                        "*/","//",],
            Hex => &[],
            Hlsl => &["/*",
                        "*/","//",],
            HolyC => &["/*",
                        "*/","//",],
            Html => &["<!--",
                        "-->",],
            Idris => &["{-",
                        "-}","--",],
            Ini => &[";","#",],
            IntelHex => &[],
            Isabelle => &["{*",
                        "*}","(*",
                        "*)","‹",
                        "›","\\<open>",
                        "\\<close>","--",],
            Jai => &["/*",
                        "*/","//",],
            Java => &["/*",
                        "*/","//",],
            JavaScript => &["/*",
                        "*/","//",],
            Json => &[],
            Jsonnet => &["/*",
                        "*/","//","#",],
            Jsx => &["/*",
                        "*/","//",],
            Julia => &["#=",
                        "=#","#",],
            Julius => &["/*",
                        "*/","//",],
            Jupyter => &[],
            K => &["/",],
            KakouneScript => &["#",],
            Kotlin => &["/*",
                        "*/","//",],
            LLVM => &[";",],
            Lean => &["/-",
                        "-/","--",],
            Less => &["/*",
                        "*/","//",],
            LinkerScript => &["/*",
                        "*/","//",],
            Liquid => &["<!--",
                        "-->","{% comment %}",
                        "{% endcomment %}",],
            Lisp => &["#|",
                        "|#",";",],
            LiveScript => &["/*",
                        "*/","#",],
            Logtalk => &["/*",
                        "*/","%",],
            Lua => &["--[[",
                        "]]","--",],
            Lucius => &["/*",
                        "*/","//",],
            Madlang => &["{#",
                        "#}","#",],
            Makefile => &["#",],
            Markdown => &[],
            Meson => &["#",],
            Mint => &[],
            ModuleDef => &[";",],
            MoonScript => &["--",],
            MsBuild => &["<!--",
                        "-->",],
            Mustache => &["{{!",
                        "}}",],
            Nim => &["#",],
            Nix => &["/*",
                        "*/","#",],
            NotQuitePerl => &["=begin",
                        "=end","#",],
            OCaml => &["(*",
                        "*)",],
            ObjectiveC => &["/*",
                        "*/","//",],
            ObjectiveCpp => &["/*",
                        "*/","//",],
            Odin => &["/*",
                        "*/","//",],
            OpenType => &["#",],
            Org => &["# ",],
            Oz => &["/*",
                        "*/","%",],
            PSL => &["/*",
                        "*/","//",],
            Pan => &["#",],
            Pascal => &["{",
                        "}","(*",
                        "*)","//",],
            Perl => &["=pod",
                        "=cut","#",],
            Perl6 => &["=begin",
                        "=end","#",],
            Pest => &["//",],
            Php => &["/*",
                        "*/","#","//",],
            Polly => &["<!--",
                        "-->",],
            Pony => &["/*",
                        "*/","//",],
            PostCss => &["/*",
                        "*/","//",],
            PowerShell => &["<#",
                        "#>","#",],
            Processing => &["/*",
                        "*/","//",],
            Prolog => &["/*",
                        "*/","%",],
            Protobuf => &["//",],
            Pug => &["//","//-",],
            PureScript => &["{-",
                        "-}","--",],
            Python => &["#",],
            Q => &["/",],
            Qcl => &["/*",
                        "*/","//",],
            Qml => &["/*",
                        "*/","//",],
            R => &["#",],
            RON => &["/*",
                        "*/","//",],
            RPMSpecfile => &["#",],
            Racket => &["#|",
                        "|#",";",],
            Rakefile => &["=begin",
                        "=end","#",],
            Razor => &["<!--",
                        "-->","@*",
                        "*@",],
            ReStructuredText => &[],
            Renpy => &["#",],
            Ruby => &["=begin",
                        "=end","#",],
            RubyHtml => &["<!--",
                        "-->",],
            Rust => &["/*",
                        "*/","//",],
            SRecode => &[";;",],
            Sass => &["/*",
                        "*/","//",],
            Scala => &["/*",
                        "*/","//",],
            Scheme => &["#|",
                        "|#",";",],
            Scons => &["#",],
            Sh => &["#",],
            Sml => &["(*",
                        "*)",],
            Solidity => &["/*",
                        "*/","//",],
            SpecmanE => &["'>",
                        "<'","--","//",],
            Spice => &["*",],
            Sql => &["/*",
                        "*/","--",],
            Stan => &["/*",
                        "*/","//","#",],
            Stratego => &["/*",
                        "*/","//",],
            Stylus => &["/*",
                        "*/","//",],
            Svelte => &["<!--",
                        "-->",],
            Svg => &["<!--",
                        "-->",],
            Swift => &["/*",
                        "*/","//",],
            Swig => &["/*",
                        "*/","//",],
            SystemVerilog => &["/*",
                        "*/","//",],
            Tcl => &["#",],
            Tera => &["<!--",
                        "-->","{#",
                        "#}",],
            Tex => &["%",],
            Text => &[],
            Thrift => &["/*",
                        "*/","#","//",],
            Toml => &["#",],
            Tsx => &["/*",
                        "*/","//",],
            Ttcn => &["/*",
                        "*/","//",],
            Twig => &["<!--",
                        "-->","{#",
                        "#}",],
            TypeScript => &["/*",
                        "*/","//",],
            UnrealDeveloperMarkdown => &[],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &["/*",
                        "*/","//",],
            UnrealShader => &["/*",
                        "*/","//",],
            UnrealShaderHeader => &["/*",
                        "*/","//",],
            UrWeb => &["(*",
                        "*)",],
            UrWebProject => &["#",],
            VB6 => &["'",],
            VBScript => &["'","REM",],
            Vala => &["/*",
                        "*/","//",],
            Velocity => &["#*",
                        "*#","##",],
            Verilog => &["/*",
                        "*/","//",],
            VerilogArgsFile => &[],
            Vhdl => &["--",],
            VimScript => &["\"",],
            VisualBasic => &["'",],
            VisualStudioProject => &["<!--",
                        "-->",],
            VisualStudioSolution => &[],
            Vue => &["<!--",
                        "-->","/*",
                        "*/","//",],
            WebAssembly => &[";;",],
            Wolfram => &["(*",
                        "*)",],
            XSL => &["<!--",
                        "-->",],
            Xaml => &["<!--",
                        "-->",],
            XcodeConfig => &["//",],
            Xml => &["<!--",
                        "-->",],
            Xtend => &["/*",
                        "*/","//",],
            Yaml => &["#",],
            Zig => &["//",],
            Zsh => &["#",],
            
        }
    }

    /// Returns the parts of syntax that determines whether tokei can skip large
    /// parts of analysis.
    pub fn important_syntax(self) -> &'static [&'static str] {
        match self {
            ABNF => &[],
            Abap => &[],
            ActionScript => &["\"","/*",],
            Ada => &[],
            Agda => &["{-",],
            Alex => &[],
            Alloy => &["/*",],
            Arduino => &["\"","/*",],
            AsciiDoc => &["////",],
            Asn1 => &["\"","'","/*",],
            Asp => &[],
            AspNet => &["<!--","<%--",],
            Assembly => &["\"","'",],
            AssemblyGAS => &["\"","/*",],
            AutoHotKey => &["/*",],
            Autoconf => &[],
            Automake => &[],
            Bash => &["\"","'",],
            Batch => &[],
            Bean => &["\"",],
            BrightScript => &["\"",],
            C => &["\"","/*",],
            CHeader => &["\"","/*",],
            CMake => &["\"",],
            CSharp => &["\"","/*",],
            CShell => &[],
            Cabal => &["{-",],
            Cassius => &["\"","'","/*",],
            Ceylon => &["\"\"\"","\"","/*",],
            Clojure => &["\"",],
            ClojureC => &["\"",],
            ClojureScript => &["\"",],
            Cobol => &[],
            CodeQL => &["\"","/*",],
            CoffeeScript => &["\"","'","###",],
            Cogent => &[],
            ColdFusion => &["\"","'","<!---",],
            ColdFusionScript => &["\"","/*",],
            Coq => &["\"","(*",],
            Cpp => &["\"","/*",],
            CppHeader => &["\"","/*",],
            Crystal => &["\"","'",],
            Css => &["\"","'","/*",],
            D => &["\"","'","/*","/+",],
            Daml => &["{-",],
            Dart => &["\"\"\"","'''","\"","'","/*",],
            DeviceTree => &["\"","/*",],
            Dhall => &["\"","''","{-",],
            Dockerfile => &["\"","'",],
            DotNetResource => &["\"","<!--",],
            DreamMaker => &["{\"","\"","'","/*",],
            Dust => &["{!",],
            Edn => &[],
            Elisp => &[],
            Elixir => &["\"\"\"","'''","\"","'",],
            Elm => &["{-",],
            Elvish => &["\"","'",],
            EmacsDevEnv => &[],
            Emojicode => &["❌🔤","💭🔜","📗","📘",],
            Erlang => &[],
            FEN => &[],
            FSharp => &["\"","(*",],
            Fish => &["\"","'",],
            FlatBuffers => &["\"","/*",],
            Forth => &["( ",],
            FortranLegacy => &["\"","'",],
            FortranModern => &["\"",],
            FreeMarker => &["<#--",],
            Fstar => &["\"","(*",],
            Futhark => &[],
            GDB => &[],
            GdScript => &["\"\"\"","\"","'",],
            Gherkin => &[],
            Gleam => &["\"",],
            Glsl => &["\"","/*",],
            Go => &["\"","/*",],
            Gohtml => &["\"","'","<!--","{{/*",],
            Graphql => &["\"\"\"","\"",],
            Groovy => &["\"","/*",],
            Gwion => &["\"",],
            Hamlet => &["\"","'","<!--",],
            Handlebars => &["\"","'","<!--","{{!",],
            Happy => &[],
            Haskell => &["{-",],
            Haxe => &["\"","'","/*",],
            Hcl => &["\"","/*",],
            Headache => &["\"","/*",],
            Hex => &[],
            Hlsl => &["\"","/*",],
            HolyC => &["\"","/*",],
            Html => &["\"","'","<!--","<script","<style",],
            Idris => &["\"\"\"","\"","{-",],
            Ini => &[],
            IntelHex => &[],
            Isabelle => &["''","{*","(*","‹","\\<open>",],
            Jai => &["\"","/*",],
            Java => &["\"","/*",],
            JavaScript => &["\"","'","`","/*",],
            Json => &[],
            Jsonnet => &["\"","'","/*",],
            Jsx => &["\"","'","`","/*",],
            Julia => &["\"\"\"","\"","#=",],
            Julius => &["\"","'","`","/*",],
            Jupyter => &[],
            K => &["\"",],
            KakouneScript => &["\"","'",],
            Kotlin => &["\"\"\"","\"","/*",],
            LLVM => &["\"","'",],
            Lean => &["/-",],
            Less => &["\"","'","/*",],
            LinkerScript => &["\"","/*",],
            Liquid => &["\"","'","<!--","{% comment %}",],
            Lisp => &["#|",],
            LiveScript => &["\"","'","/*",],
            Logtalk => &["\"","/*",],
            Lua => &["\"","'","--[[",],
            Lucius => &["\"","'","/*",],
            Madlang => &["{#",],
            Makefile => &[],
            Markdown => &["```",],
            Meson => &["'''","'",],
            Mint => &[],
            ModuleDef => &[],
            MoonScript => &["\"","'",],
            MsBuild => &["\"","'","<!--",],
            Mustache => &["\"","'","{{!",],
            Nim => &["\"\"\"","\"",],
            Nix => &["\"","/*",],
            NotQuitePerl => &["\"","'","=begin",],
            OCaml => &["\"","(*",],
            ObjectiveC => &["\"","/*",],
            ObjectiveCpp => &["\"","/*",],
            Odin => &["\"","'","/*",],
            OpenType => &[],
            Org => &[],
            Oz => &["\"","/*",],
            PSL => &["\"","/*",],
            Pan => &["\"","'",],
            Pascal => &["'","{","(*",],
            Perl => &["\"","'","=pod",],
            Perl6 => &["\"","'","=begin",],
            Pest => &["\"","'",],
            Php => &["\"","'","/*",],
            Polly => &["\"","'","<!--",],
            Pony => &["\"","\"\"\"","/*",],
            PostCss => &["\"","'","/*",],
            PowerShell => &["\"@","\"","@'","'","<#",],
            Processing => &["\"","/*",],
            Prolog => &["\"","/*",],
            Protobuf => &[],
            Pug => &["#{\"","#{'","#{`",],
            PureScript => &["{-",],
            Python => &["\"","'","\"\"\"","'''",],
            Q => &["\"",],
            Qcl => &["\"","/*",],
            Qml => &["\"","'","/*",],
            R => &[],
            RON => &["\"","/*",],
            RPMSpecfile => &[],
            Racket => &["#|",],
            Rakefile => &["\"","'","=begin",],
            Razor => &["<!--","@*",],
            ReStructuredText => &[],
            Renpy => &["\"","'","`",],
            Ruby => &["\"","'","=begin",],
            RubyHtml => &["\"","'","<!--","<script","<style",],
            Rust => &["#\"","\"","/*","///","//!",],
            SRecode => &[],
            Sass => &["\"","'","/*",],
            Scala => &["\"","/*",],
            Scheme => &["#|",],
            Scons => &["\"\"\"","'''","\"","'",],
            Sh => &["\"","'",],
            Sml => &["\"","(*",],
            Solidity => &["\"","/*",],
            SpecmanE => &["'>",],
            Spice => &[],
            Sql => &["'","/*",],
            Stan => &["\"","/*",],
            Stratego => &["\"","$[","$<","${","/*",],
            Stylus => &["\"","'","/*",],
            Svelte => &["\"","'","<!--","<script","<style",],
            Svg => &["\"","'","<!--",],
            Swift => &["\"","/*",],
            Swig => &["\"","/*",],
            SystemVerilog => &["\"","/*",],
            Tcl => &["\"","'",],
            Tera => &["\"","'","<!--","{#",],
            Tex => &[],
            Text => &[],
            Thrift => &["\"","'","/*",],
            Toml => &["\"\"\"","'''","\"","'",],
            Tsx => &["\"","'","`","/*",],
            Ttcn => &["\"","/*",],
            Twig => &["\"","'","<!--","{#",],
            TypeScript => &["\"","'","`","/*",],
            UnrealDeveloperMarkdown => &["```",],
            UnrealPlugin => &[],
            UnrealProject => &[],
            UnrealScript => &["\"","/*",],
            UnrealShader => &["\"","/*",],
            UnrealShaderHeader => &["\"","/*",],
            UrWeb => &["\"","(*",],
            UrWebProject => &[],
            VB6 => &[],
            VBScript => &[],
            Vala => &["\"","/*",],
            Velocity => &["\"","'","#*",],
            Verilog => &["\"","/*",],
            VerilogArgsFile => &[],
            Vhdl => &[],
            VimScript => &["\"","'",],
            VisualBasic => &["\"",],
            VisualStudioProject => &["\"","'","<!--",],
            VisualStudioSolution => &[],
            Vue => &["\"","'","`","<!--","/*","<script","<style","<template",],
            WebAssembly => &["\"","'",],
            Wolfram => &["\"","(*",],
            XSL => &["\"","'","<!--",],
            Xaml => &["\"","'","<!--",],
            XcodeConfig => &["\"","'",],
            Xml => &["\"","'","<!--",],
            Xtend => &["'''","\"","'","/*",],
            Yaml => &["\"","'",],
            Zig => &["\"",],
            Zsh => &["\"","'",],
            
        }
    }

    /// Get language from a file path. May open and read the file.
    ///
    /// ```no_run
    /// use tokei::{Config, LanguageType};
    ///
    /// let rust = LanguageType::from_path("./main.rs", &Config::default());
    ///
    /// assert_eq!(rust, Some(LanguageType::Rust));
    /// ```
    pub fn from_path<P: AsRef<Path>>(entry: P, _config: &Config)
        -> Option<Self>
    {
        let entry = entry.as_ref();

        if let Some(filename) = fsutils::get_filename(&entry) {
            match &*filename {
                | "cmakelists.txt"=> return Some(CMake),
                    | "dockerfile"=> return Some(Dockerfile),
                    | "makefile"=> return Some(Makefile),
                    | "meson.build"| "meson_options.txt"=> return Some(Meson),
                    | "rakefile"=> return Some(Rakefile),
                    | "sconstruct"| "sconscript"=> return Some(Scons),
                    
                _ => ()
            }
        }

        match fsutils::get_extension(&entry) {
            Some(extension) => LanguageType::from_file_extension(extension.as_str()),
            None => LanguageType::from_shebang(&entry),
        }
    }

    /// Get language from a file extension.
    ///
    /// ```no_run
    /// use tokei::LanguageType;
    ///
    /// let rust = LanguageType::from_file_extension("rs");
    ///
    /// assert_eq!(rust, Some(LanguageType::Rust));
    /// ```
    pub fn from_file_extension(extension: &str) -> Option<Self> {
        match extension {
            | "abnf" => Some(ABNF),
                | "abap" => Some(Abap),
                | "as" => Some(ActionScript),
                | "ada" | "adb" | "ads" | "pad" => Some(Ada),
                | "agda" => Some(Agda),
                | "x" => Some(Alex),
                | "als" => Some(Alloy),
                | "ino" => Some(Arduino),
                | "adoc" | "asciidoc" => Some(AsciiDoc),
                | "asn1" => Some(Asn1),
                | "asa" | "asp" => Some(Asp),
                | "asax" | "ascx" | "asmx" | "aspx" | "master" | "sitemap" | "webinfo" => Some(AspNet),
                | "asm" => Some(Assembly),
                | "s" => Some(AssemblyGAS),
                | "ahk" => Some(AutoHotKey),
                | "in" => Some(Autoconf),
                | "am" => Some(Automake),
                | "bash" => Some(Bash),
                | "bat" | "btm" | "cmd" => Some(Batch),
                | "bean" | "beancount" => Some(Bean),
                | "brs" => Some(BrightScript),
                | "c" | "ec" | "pgc" => Some(C),
                | "h" => Some(CHeader),
                | "cmake" => Some(CMake),
                | "cs" | "csx" => Some(CSharp),
                | "csh" => Some(CShell),
                | "cabal" => Some(Cabal),
                | "cassius" => Some(Cassius),
                | "ceylon" => Some(Ceylon),
                | "clj" => Some(Clojure),
                | "cljc" => Some(ClojureC),
                | "cljs" => Some(ClojureScript),
                | "cob" | "cbl" | "ccp" | "cobol" | "cpy" => Some(Cobol),
                | "ql" | "qll" => Some(CodeQL),
                | "coffee" | "cjsx" => Some(CoffeeScript),
                | "cogent" => Some(Cogent),
                | "cfm" => Some(ColdFusion),
                | "cfc" => Some(ColdFusionScript),
                | "v" => Some(Coq),
                | "cc" | "cpp" | "cxx" | "c++" | "pcc" | "tpp" => Some(Cpp),
                | "hh" | "hpp" | "hxx" | "inl" | "ipp" => Some(CppHeader),
                | "cr" => Some(Crystal),
                | "css" => Some(Css),
                | "d" => Some(D),
                | "daml" => Some(Daml),
                | "dart" => Some(Dart),
                | "dts" | "dtsi" => Some(DeviceTree),
                | "dhall" => Some(Dhall),
                | "dockerfile" | "dockerignore" => Some(Dockerfile),
                | "resx" => Some(DotNetResource),
                | "dm" | "dme" => Some(DreamMaker),
                | "dust" => Some(Dust),
                | "edn" => Some(Edn),
                | "el" => Some(Elisp),
                | "ex" | "exs" => Some(Elixir),
                | "elm" => Some(Elm),
                | "elv" => Some(Elvish),
                | "ede" => Some(EmacsDevEnv),
                | "emojic" | "🍇" => Some(Emojicode),
                | "erl" | "hrl" => Some(Erlang),
                | "fen" => Some(FEN),
                | "fs" | "fsi" | "fsx" | "fsscript" => Some(FSharp),
                | "fish" => Some(Fish),
                | "fbs" => Some(FlatBuffers),
                | "4th" | "forth" | "fr" | "frt" | "fth" | "f83" | "fb" | "fpm" | "e4" | "rx" | "ft" => Some(Forth),
                | "f" | "for" | "ftn" | "f77" | "pfo" => Some(FortranLegacy),
                | "f03" | "f08" | "f90" | "f95" => Some(FortranModern),
                | "ftl" | "ftlh" | "ftlx" => Some(FreeMarker),
                | "fst" => Some(Fstar),
                | "fut" => Some(Futhark),
                | "gdb" => Some(GDB),
                | "gd" => Some(GdScript),
                | "feature" => Some(Gherkin),
                | "gleam" => Some(Gleam),
                | "vert" | "tesc" | "tese" | "geom" | "frag" | "comp" | "glsl" => Some(Glsl),
                | "go" => Some(Go),
                | "gohtml" => Some(Gohtml),
                | "gql" | "graphql" => Some(Graphql),
                | "groovy" | "grt" | "gtpl" | "gvy" => Some(Groovy),
                | "gw" => Some(Gwion),
                | "hamlet" => Some(Hamlet),
                | "hbs" | "handlebars" => Some(Handlebars),
                | "y" | "ly" => Some(Happy),
                | "hs" => Some(Haskell),
                | "hx" => Some(Haxe),
                | "tf" | "tfvars" => Some(Hcl),
                | "ha" => Some(Headache),
                | "hex" => Some(Hex),
                | "hlsl" => Some(Hlsl),
                | "HC" | "hc" => Some(HolyC),
                | "html" | "htm" => Some(Html),
                | "idr" | "lidr" => Some(Idris),
                | "ini" => Some(Ini),
                | "ihex" => Some(IntelHex),
                | "thy" => Some(Isabelle),
                | "jai" => Some(Jai),
                | "java" => Some(Java),
                | "js" | "mjs" => Some(JavaScript),
                | "json" => Some(Json),
                | "jsonnet" | "libsonnet" => Some(Jsonnet),
                | "jsx" => Some(Jsx),
                | "jl" => Some(Julia),
                | "julius" => Some(Julius),
                | "ipynb" => Some(Jupyter),
                | "k" => Some(K),
                | "kak" => Some(KakouneScript),
                | "kt" | "kts" => Some(Kotlin),
                | "ll" => Some(LLVM),
                | "lean" | "hlean" => Some(Lean),
                | "less" => Some(Less),
                | "lds" => Some(LinkerScript),
                | "liquid" => Some(Liquid),
                | "lisp" | "lsp" => Some(Lisp),
                | "ls" => Some(LiveScript),
                | "lgt" | "logtalk" => Some(Logtalk),
                | "lua" => Some(Lua),
                | "lucius" => Some(Lucius),
                | "mad" => Some(Madlang),
                | "makefile" | "mak" | "mk" => Some(Makefile),
                | "md" | "markdown" => Some(Markdown),
                | "mint" => Some(Mint),
                | "def" => Some(ModuleDef),
                | "moon" => Some(MoonScript),
                | "csproj" | "vbproj" | "fsproj" | "props" | "targets" => Some(MsBuild),
                | "mustache" => Some(Mustache),
                | "nim" => Some(Nim),
                | "nix" => Some(Nix),
                | "nqp" => Some(NotQuitePerl),
                | "ml" | "mli" | "mll" | "mly" | "re" | "rei" => Some(OCaml),
                | "m" => Some(ObjectiveC),
                | "mm" => Some(ObjectiveCpp),
                | "odin" => Some(Odin),
                | "fea" => Some(OpenType),
                | "org" => Some(Org),
                | "oz" => Some(Oz),
                | "psl" => Some(PSL),
                | "pan" | "tpl" => Some(Pan),
                | "pas" | "pp" => Some(Pascal),
                | "pl" | "pm" => Some(Perl),
                | "pl6" | "pm6" => Some(Perl6),
                | "pest" => Some(Pest),
                | "php" => Some(Php),
                | "polly" => Some(Polly),
                | "pony" => Some(Pony),
                | "pcss" | "sss" => Some(PostCss),
                | "ps1" | "psm1" | "psd1" | "ps1xml" | "cdxml" | "pssc" | "psc1" => Some(PowerShell),
                | "pde" => Some(Processing),
                | "p" | "pro" => Some(Prolog),
                | "proto" => Some(Protobuf),
                | "pug" => Some(Pug),
                | "purs" => Some(PureScript),
                | "py" | "pyw" => Some(Python),
                | "q" => Some(Q),
                | "qcl" => Some(Qcl),
                | "qml" => Some(Qml),
                | "r" => Some(R),
                | "ron" => Some(RON),
                | "spec" => Some(RPMSpecfile),
                | "rkt" => Some(Racket),
                | "rake" => Some(Rakefile),
                | "cshtml" => Some(Razor),
                | "rst" => Some(ReStructuredText),
                | "rpy" => Some(Renpy),
                | "rb" => Some(Ruby),
                | "rhtml" | "erb" => Some(RubyHtml),
                | "rs" => Some(Rust),
                | "srt" => Some(SRecode),
                | "sass" | "scss" => Some(Sass),
                | "sc" | "scala" => Some(Scala),
                | "scm" | "ss" => Some(Scheme),
                | "sh" => Some(Sh),
                | "sml" => Some(Sml),
                | "sol" => Some(Solidity),
                | "e" => Some(SpecmanE),
                | "ckt" => Some(Spice),
                | "sql" => Some(Sql),
                | "stan" => Some(Stan),
                | "str" => Some(Stratego),
                | "styl" => Some(Stylus),
                | "svelte" => Some(Svelte),
                | "svg" => Some(Svg),
                | "swift" => Some(Swift),
                | "swg" | "i" => Some(Swig),
                | "sv" | "svh" => Some(SystemVerilog),
                | "tcl" => Some(Tcl),
                | "tera" => Some(Tera),
                | "tex" | "sty" => Some(Tex),
                | "text" | "txt" => Some(Text),
                | "thrift" => Some(Thrift),
                | "toml" => Some(Toml),
                | "tsx" => Some(Tsx),
                | "ttcn" | "ttcn3" | "ttcnpp" => Some(Ttcn),
                | "twig" => Some(Twig),
                | "ts" => Some(TypeScript),
                | "udn" => Some(UnrealDeveloperMarkdown),
                | "uplugin" => Some(UnrealPlugin),
                | "uproject" => Some(UnrealProject),
                | "uc" | "uci" | "upkg" => Some(UnrealScript),
                | "usf" => Some(UnrealShader),
                | "ush" => Some(UnrealShaderHeader),
                | "ur" | "urs" => Some(UrWeb),
                | "urp" => Some(UrWebProject),
                | "frm" | "bas" | "cls" => Some(VB6),
                | "vbs" => Some(VBScript),
                | "vala" => Some(Vala),
                | "vm" => Some(Velocity),
                | "vg" | "vh" => Some(Verilog),
                | "irunargs" | "xrunargs" => Some(VerilogArgsFile),
                | "vhd" | "vhdl" => Some(Vhdl),
                | "vim" => Some(VimScript),
                | "vb" => Some(VisualBasic),
                | "vcproj" | "vcxproj" => Some(VisualStudioProject),
                | "sln" => Some(VisualStudioSolution),
                | "vue" => Some(Vue),
                | "wat" | "wast" => Some(WebAssembly),
                | "nb" | "wl" => Some(Wolfram),
                | "xsl" | "xslt" => Some(XSL),
                | "xaml" => Some(Xaml),
                | "xcconfig" => Some(XcodeConfig),
                | "xml" => Some(Xml),
                | "xtend" => Some(Xtend),
                | "yaml" | "yml" => Some(Yaml),
                | "zig" => Some(Zig),
                | "zsh" => Some(Zsh),
                
            extension => {
                warn!("Unknown extension: {}", extension);
                None
            },
        }
    }

    /// Get language from its MIME type if available.
    ///
    /// ```no_run
    /// use tokei::LanguageType;
    ///
    /// let lang = LanguageType::from_mime("application/javascript");
    ///
    /// assert_eq!(lang, Some(LanguageType::JavaScript));
    /// ```
    pub fn from_mime(mime: &str) -> Option<Self> {
        match mime {
            | "text/css" => Some(Css),
                | "text/html" => Some(Html),
                | "application/javascript" | "application/ecmascript" | "application/x-ecmascript" | "application/x-javascript" | "text/javascript" | "text/ecmascript" | "text/javascript1.0" | "text/javascript1.1" | "text/javascript1.2" | "text/javascript1.3" | "text/javascript1.4" | "text/javascript1.5" | "text/jscript" | "text/livescript" | "text/x-ecmascript" | "text/x-javascript" => Some(JavaScript),
                | "application/json" | "application/manifest+json" => Some(Json),
                | "text/x-python" => Some(Python),
                | "image/svg+xml" => Some(Svg),
                | "text/plain" => Some(Text),
                
            _ => {
                warn!("Unknown MIME: {}", mime);
                None
            },
        }
    }

    /// Get language from a shebang. May open and read the file.
    ///
    /// ```no_run
    /// use tokei::LanguageType;
    ///
    /// let rust = LanguageType::from_shebang("./main.rs");
    ///
    /// assert_eq!(rust, Some(LanguageType::Rust));
    /// ```
    pub fn from_shebang<P: AsRef<Path>>(entry: P) -> Option<Self> {
        let file = match File::open(entry) {
            Ok(file) => file,
            _ => return None,
        };

        let mut buf = BufReader::new(file);
        let mut line = String::new();
        let _ = buf.read_line(&mut line);

        let mut words = line.split_whitespace();
        match words.next() {
            
            | Some("#!/bin/bash") => Some(Bash),
                | Some("#!/bin/csh") => Some(CShell),
                | Some("#!/usr/bin/crystal") => Some(Crystal),
                | Some("#!/bin/fish") => Some(Fish),
                | Some("#!/usr/bin/perl") => Some(Perl),
                | Some("#!/bin/sh") => Some(Sh),
                | Some("#!/bin/zsh") => Some(Zsh),
                

            Some("#!/usr/bin/env") => {
                if let Some(word) = words.next() {
                    match word {
                        | "bash" => Some(Bash),
                            | "csh" => Some(CShell),
                            | "crystal" => Some(Crystal),
                            | "elvish" => Some(Elvish),
                            | "fish" => Some(Fish),
                            | "python" | "python2" | "python3" => Some(Python),
                            | "ruby" => Some(Ruby),
                            | "sh" => Some(Sh),
                            
                        env => {
                            warn!("Unknown environment: {:?}", env);
                            None
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl FromStr for LanguageType {
    type Err = &'static str;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match &*from.to_lowercase() {
            
                "abnf"
                => Ok(ABNF),
            
                "abap"
                => Ok(Abap),
            
                "actionscript"
                => Ok(ActionScript),
            
                "ada"
                => Ok(Ada),
            
                "agda"
                => Ok(Agda),
            
                "alex"
                => Ok(Alex),
            
                "alloy"
                => Ok(Alloy),
            
                "arduino c++"
                => Ok(Arduino),
            
                "asciidoc"
                => Ok(AsciiDoc),
            
                "asn.1"
                => Ok(Asn1),
            
                "asp"
                => Ok(Asp),
            
                "asp.net"
                => Ok(AspNet),
            
                "assembly"
                => Ok(Assembly),
            
                "gnu style assembly"
                => Ok(AssemblyGAS),
            
                "autohotkey"
                => Ok(AutoHotKey),
            
                "autoconf"
                => Ok(Autoconf),
            
                "automake"
                => Ok(Automake),
            
                "bash"
                => Ok(Bash),
            
                "batch"
                => Ok(Batch),
            
                "bean"
                => Ok(Bean),
            
                "brightscript"
                => Ok(BrightScript),
            
                "c"
                => Ok(C),
            
                "c header"
                => Ok(CHeader),
            
                "cmake"
                => Ok(CMake),
            
                "c#"
                => Ok(CSharp),
            
                "c shell"
                => Ok(CShell),
            
                "cabal"
                => Ok(Cabal),
            
                "cassius"
                => Ok(Cassius),
            
                "ceylon"
                => Ok(Ceylon),
            
                "clojure"
                => Ok(Clojure),
            
                "clojurec"
                => Ok(ClojureC),
            
                "clojurescript"
                => Ok(ClojureScript),
            
                "cobol"
                => Ok(Cobol),
            
                "codeql"
                => Ok(CodeQL),
            
                "coffeescript"
                => Ok(CoffeeScript),
            
                "cogent"
                => Ok(Cogent),
            
                "coldfusion"
                => Ok(ColdFusion),
            
                "coldfusion cfscript"
                => Ok(ColdFusionScript),
            
                "coq"
                => Ok(Coq),
            
                "c++"
                => Ok(Cpp),
            
                "c++ header"
                => Ok(CppHeader),
            
                "crystal"
                => Ok(Crystal),
            
                "css"
                => Ok(Css),
            
                "d"
                => Ok(D),
            
                "daml"
                => Ok(Daml),
            
                "dart"
                => Ok(Dart),
            
                "device tree"
                => Ok(DeviceTree),
            
                "dhall"
                => Ok(Dhall),
            
                "dockerfile"
                => Ok(Dockerfile),
            
                ".net resource"
                => Ok(DotNetResource),
            
                "dream maker"
                => Ok(DreamMaker),
            
                "dust.js"
                => Ok(Dust),
            
                "edn"
                => Ok(Edn),
            
                "emacs lisp"
                => Ok(Elisp),
            
                "elixir"
                => Ok(Elixir),
            
                "elm"
                => Ok(Elm),
            
                "elvish"
                => Ok(Elvish),
            
                "emacs dev env"
                => Ok(EmacsDevEnv),
            
                "emojicode"
                => Ok(Emojicode),
            
                "erlang"
                => Ok(Erlang),
            
                "fen"
                => Ok(FEN),
            
                "f#"
                => Ok(FSharp),
            
                "fish"
                => Ok(Fish),
            
                "flatbuffers schema"
                => Ok(FlatBuffers),
            
                "forth"
                => Ok(Forth),
            
                "fortran legacy"
                => Ok(FortranLegacy),
            
                "fortran modern"
                => Ok(FortranModern),
            
                "freemarker"
                => Ok(FreeMarker),
            
                "f*"
                => Ok(Fstar),
            
                "futhark"
                => Ok(Futhark),
            
                "gdb script"
                => Ok(GDB),
            
                "gdscript"
                => Ok(GdScript),
            
                "gherkin (cucumber)"
                => Ok(Gherkin),
            
                "gleam"
                => Ok(Gleam),
            
                "glsl"
                => Ok(Glsl),
            
                "go"
                => Ok(Go),
            
                "go html"
                => Ok(Gohtml),
            
                "graphql"
                => Ok(Graphql),
            
                "groovy"
                => Ok(Groovy),
            
                "gwion"
                => Ok(Gwion),
            
                "hamlet"
                => Ok(Hamlet),
            
                "handlebars"
                => Ok(Handlebars),
            
                "happy"
                => Ok(Happy),
            
                "haskell"
                => Ok(Haskell),
            
                "haxe"
                => Ok(Haxe),
            
                "hcl"
                => Ok(Hcl),
            
                "headache"
                => Ok(Headache),
            
                "hex"
                => Ok(Hex),
            
                "hlsl"
                => Ok(Hlsl),
            
                "holyc"
                => Ok(HolyC),
            
                "html"
                => Ok(Html),
            
                "idris"
                => Ok(Idris),
            
                "ini"
                => Ok(Ini),
            
                "intel hex"
                => Ok(IntelHex),
            
                "isabelle"
                => Ok(Isabelle),
            
                "jai"
                => Ok(Jai),
            
                "java"
                => Ok(Java),
            
                "javascript"
                => Ok(JavaScript),
            
                "json"
                => Ok(Json),
            
                "jsonnet"
                => Ok(Jsonnet),
            
                "jsx"
                => Ok(Jsx),
            
                "julia"
                => Ok(Julia),
            
                "julius"
                => Ok(Julius),
            
                "jupyter notebooks"
                => Ok(Jupyter),
            
                "k"
                => Ok(K),
            
                "kakoune script"
                => Ok(KakouneScript),
            
                "kotlin"
                => Ok(Kotlin),
            
                "llvm"
                => Ok(LLVM),
            
                "lean"
                => Ok(Lean),
            
                "less"
                => Ok(Less),
            
                "ld script"
                => Ok(LinkerScript),
            
                "liquid"
                => Ok(Liquid),
            
                "lisp"
                => Ok(Lisp),
            
                "livescript"
                => Ok(LiveScript),
            
                "logtalk"
                => Ok(Logtalk),
            
                "lua"
                => Ok(Lua),
            
                "lucius"
                => Ok(Lucius),
            
                "madlang"
                => Ok(Madlang),
            
                "makefile"
                => Ok(Makefile),
            
                "markdown"
                => Ok(Markdown),
            
                "meson"
                => Ok(Meson),
            
                "mint"
                => Ok(Mint),
            
                "module-definition"
                => Ok(ModuleDef),
            
                "moonscript"
                => Ok(MoonScript),
            
                "msbuild"
                => Ok(MsBuild),
            
                "mustache"
                => Ok(Mustache),
            
                "nim"
                => Ok(Nim),
            
                "nix"
                => Ok(Nix),
            
                "not quite perl"
                => Ok(NotQuitePerl),
            
                "ocaml"
                => Ok(OCaml),
            
                "objective-c"
                => Ok(ObjectiveC),
            
                "objective-c++"
                => Ok(ObjectiveCpp),
            
                "odin"
                => Ok(Odin),
            
                "opentype feature file"
                => Ok(OpenType),
            
                "org"
                => Ok(Org),
            
                "oz"
                => Ok(Oz),
            
                "psl assertion"
                => Ok(PSL),
            
                "pan"
                => Ok(Pan),
            
                "pascal"
                => Ok(Pascal),
            
                "perl"
                => Ok(Perl),
            
                "rakudo"
                => Ok(Perl6),
            
                "pest"
                => Ok(Pest),
            
                "php"
                => Ok(Php),
            
                "polly"
                => Ok(Polly),
            
                "pony"
                => Ok(Pony),
            
                "postcss"
                => Ok(PostCss),
            
                "powershell"
                => Ok(PowerShell),
            
                "processing"
                => Ok(Processing),
            
                "prolog"
                => Ok(Prolog),
            
                "protocol buffers"
                => Ok(Protobuf),
            
                "pug"
                => Ok(Pug),
            
                "purescript"
                => Ok(PureScript),
            
                "python"
                => Ok(Python),
            
                "q"
                => Ok(Q),
            
                "qcl"
                => Ok(Qcl),
            
                "qml"
                => Ok(Qml),
            
                "r"
                => Ok(R),
            
                "rusty object notation"
                => Ok(RON),
            
                "rpm specfile"
                => Ok(RPMSpecfile),
            
                "racket"
                => Ok(Racket),
            
                "rakefile"
                => Ok(Rakefile),
            
                "razor"
                => Ok(Razor),
            
                "restructuredtext"
                => Ok(ReStructuredText),
            
                "ren'py"
                => Ok(Renpy),
            
                "ruby"
                => Ok(Ruby),
            
                "ruby html"
                => Ok(RubyHtml),
            
                "rust"
                => Ok(Rust),
            
                "srecode template"
                => Ok(SRecode),
            
                "sass"
                => Ok(Sass),
            
                "scala"
                => Ok(Scala),
            
                "scheme"
                => Ok(Scheme),
            
                "scons"
                => Ok(Scons),
            
                "shell"
                => Ok(Sh),
            
                "standard ml (sml)"
                => Ok(Sml),
            
                "solidity"
                => Ok(Solidity),
            
                "specman e"
                => Ok(SpecmanE),
            
                "spice netlist"
                => Ok(Spice),
            
                "sql"
                => Ok(Sql),
            
                "stan"
                => Ok(Stan),
            
                "stratego/xt"
                => Ok(Stratego),
            
                "stylus"
                => Ok(Stylus),
            
                "svelte"
                => Ok(Svelte),
            
                "svg"
                => Ok(Svg),
            
                "swift"
                => Ok(Swift),
            
                "swig"
                => Ok(Swig),
            
                "systemverilog"
                => Ok(SystemVerilog),
            
                "tcl"
                => Ok(Tcl),
            
                "tera"
                => Ok(Tera),
            
                "tex"
                => Ok(Tex),
            
                "plain text"
                => Ok(Text),
            
                "thrift"
                => Ok(Thrift),
            
                "toml"
                => Ok(Toml),
            
                "tsx"
                => Ok(Tsx),
            
                "ttcn-3"
                => Ok(Ttcn),
            
                "twig"
                => Ok(Twig),
            
                "typescript"
                => Ok(TypeScript),
            
                "unreal markdown"
                => Ok(UnrealDeveloperMarkdown),
            
                "unreal plugin"
                => Ok(UnrealPlugin),
            
                "unreal project"
                => Ok(UnrealProject),
            
                "unreal script"
                => Ok(UnrealScript),
            
                "unreal shader"
                => Ok(UnrealShader),
            
                "unreal shader header"
                => Ok(UnrealShaderHeader),
            
                "ur/web"
                => Ok(UrWeb),
            
                "ur/web project"
                => Ok(UrWebProject),
            
                "vb6"
                => Ok(VB6),
            
                "vbscript"
                => Ok(VBScript),
            
                "vala"
                => Ok(Vala),
            
                "apache velocity"
                => Ok(Velocity),
            
                "verilog"
                => Ok(Verilog),
            
                "verilog args file"
                => Ok(VerilogArgsFile),
            
                "vhdl"
                => Ok(Vhdl),
            
                "vim script"
                => Ok(VimScript),
            
                "visual basic"
                => Ok(VisualBasic),
            
                "visual studio project"
                => Ok(VisualStudioProject),
            
                "visual studio solution"
                => Ok(VisualStudioSolution),
            
                "vue"
                => Ok(Vue),
            
                "webassembly"
                => Ok(WebAssembly),
            
                "wolfram"
                => Ok(Wolfram),
            
                "xsl"
                => Ok(XSL),
            
                "xaml"
                => Ok(Xaml),
            
                "xcode config"
                => Ok(XcodeConfig),
            
                "xml"
                => Ok(Xml),
            
                "xtend"
                => Ok(Xtend),
            
                "yaml"
                => Ok(Yaml),
            
                "zig"
                => Ok(Zig),
            
                "zsh"
                => Ok(Zsh),
            
            _ => Err("Language not found, please use `-l` to see all available\
                     languages."),
        }
    }
}

impl fmt::Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}


impl<'a> From<LanguageType> for Cow<'a, LanguageType> {
    fn from(from: LanguageType) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a LanguageType> for Cow<'a, LanguageType> {
    fn from(from: &'a LanguageType) -> Self {
        Cow::Borrowed(from)
    }
}

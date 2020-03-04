use crate::{default_naming, Deserialize, NamingConvention};

#[derive(Debug, Clone, Deserialize)]
struct MsgDefInner {
    code: u8,
    name: String,
    #[serde(rename = "type")]
    ty: String,
    stages: Vec<String>,
    desc: String,
    fields: Vec<MsgFieldDef>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "MsgDefInner")]
pub struct MsgDef {
    inner: MsgDefInner,
    name: String,
    naming: &'static NamingConvention,
}

impl MsgDef {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn ty(&self) -> &str {
        self.inner.ty.as_ref()
    }

    pub fn desc(&self) -> &str {
        self.inner.desc.as_ref()
    }

    pub fn stages(&self) -> &[String] {
        self.inner.stages.as_ref()
    }

    pub fn kind_code(&self) -> u8 {
        self.inner.code
    }

    pub fn kind_name(&self) -> &str {
        self.inner.name.as_ref()
    }

    pub fn field_iter(&self) -> impl ExactSizeIterator<Item = &MsgFieldDef> {
        self.inner.fields.iter()
    }

    pub fn rename(&mut self, naming: &'static NamingConvention) {
        if self.naming == naming {
            return;
        }
        self.name = (naming.msg_name)(self.name.as_ref());
        self.inner.ty = (naming.msg_type)(self.inner.ty.as_ref());
        for field in self.inner.fields.iter_mut() {
            field.rename(naming);
        }
        self.naming = naming;
    }
}

impl From<MsgDefInner> for MsgDef {
    fn from(inner: MsgDefInner) -> Self {
        let name = inner.name.clone();
        Self {
            name,
            inner,
            naming: default_naming(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Deserialize)]
pub struct MsgFieldDef {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    desc: String,
    #[serde(default = "default_naming", skip)]
    naming: &'static NamingConvention,
}

impl MsgFieldDef {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn ty(&self) -> &str {
        self.ty.as_ref()
    }

    pub fn desc(&self) -> &str {
        self.desc.as_ref()
    }

    pub fn rename(&mut self, naming: &'static NamingConvention) {
        if self.naming == naming {
            return;
        }
        self.name = (naming.msg_field_name)(self.name.as_ref());
        self.ty = (naming.msg_field_type)(self.ty.as_ref());
        self.naming = naming;
    }
}

use std::fmt::{Debug, Write};
use super::TrieNode;

enum NodeData<T> {
    Solid(T),
    Root,
}

impl<T> Debug for NodeData<T>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid(data) => data.fmt(f),
            Self::Root => f.write_char('/'),
        }
    }
}
impl<T> From<T> for NodeData<T> {
    fn from(value: T) -> Self {
        Self::Solid(value)
    }
}

impl<T> Debug for TrieNode<T>
where T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const INDENT: &str = "    ";
        const CR: &str = "\n";
        const SEP: &str = ",";
        const OPEN: char = '{';
        const CLOSE: char = '}';
        const OPEN_STOP: char = '[';
        const CLOSE_STOP: char = ']';
        const OPEN_NOSTOP: char = '(';
        const CLOSE_NOSTOP: char = ')';
        const COLON: &str = ":";

        fn do_fmt<T>(
            self_: &TrieNode<T>,
            self_data: NodeData<&T>,
            f: &mut std::fmt::Formatter<'_>,
            level: usize,
            ) -> std::fmt::Result
        where T: Debug
        {
            let (indent, nindent, sep, cr, colon)
                = if f.alternate() {
                    (
                        INDENT.repeat(level),
                        INDENT.repeat(level + 1),
                        format!("{} ", SEP),
                        CR,
                        format!("{} ", COLON),
                        )
                } else {
                    (
                        "".to_string(),
                        "".to_string(),
                        SEP.to_string(),
                        "",
                        COLON.to_string(),
                        )
                };
            if self_.stop() {
                // is stop node
                f.write_char(OPEN_STOP)?;
                self_data.fmt(f)?;
                f.write_char(CLOSE_STOP)?;
            } else {
                // is not stop node
                f.write_char(OPEN_NOSTOP)?;
                self_data.fmt(f)?;
                f.write_char(CLOSE_NOSTOP)?;
            }
            f.write_str(&colon)?;
            f.write_char(OPEN)?;
            let mut childs
                = self_.childs().iter().peekable();
            if childs.peek().is_none() {
                // no element
                return f.write_char(CLOSE)
            }
            f.write_str(cr)?;
            while let Some((data, node)) = childs.next() {
                f.write_str(&nindent)?;
                do_fmt(node, data.into(), f, level + 1)?;
                if childs.peek().is_some() {
                    // 非即将结束
                    f.write_str(&sep)?;
                }
                f.write_str(cr)?;
            }
            f.write_str(&indent)?;
            f.write_char(CLOSE)?;
            Ok(())
        }
        do_fmt(self, NodeData::Root, f, 0)
    }
}

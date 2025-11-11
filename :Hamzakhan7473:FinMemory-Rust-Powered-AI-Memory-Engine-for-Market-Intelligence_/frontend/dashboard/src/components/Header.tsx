interface HeaderProps {
  title: string;
}

export function Header({ title }: HeaderProps) {
  return (
    <header className="header">
      <h1>{title}</h1>
      <p className="subtitle">Rust-powered intelligence for market research teams</p>
    </header>
  );
}

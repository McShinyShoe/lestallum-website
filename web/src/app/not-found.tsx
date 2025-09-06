import { Button } from "@/components/ui/button";

export default function NotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-background">
      <h1 className="text-9xl font-bold text-muted-foreground">404</h1>
      <p className="mt-4 mb-2 text-2xl text-ring">Page not found</p>
      <a href="/">
        <Button>Go back home</Button>
      </a>
    </div>
  );
}
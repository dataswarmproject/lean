import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export function middleware(request: NextRequest) {
  const hostname = request.headers.get('host') || '';
  const url = request.nextUrl.clone();

  // Extract subdomain
  const subdomain = hostname.split('.')[0];

  // Map subdomains to routes
  const subdomainRoutes: Record<string, string> = {
    'forum': '/forum',
    'docs': '/docs',
    'validators': '/validators',
    'explorer': '/explorer',
    'staking': '/staking',
    'status': '/status',
  };

  // If it's a subdomain and not the main domain
  if (subdomain && subdomain !== 'www' && subdomain !== 'leanchain' && subdomainRoutes[subdomain]) {
    // Rewrite to the appropriate route
    url.pathname = subdomainRoutes[subdomain] + url.pathname;
    return NextResponse.rewrite(url);
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - api (API routes)
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     */
    '/((?!api|_next/static|_next/image|favicon.ico).*)',
  ],
};


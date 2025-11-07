import type { Metadata } from 'next';
import { Space_Grotesk } from 'next/font/google';
import './globals.css';
import { ThemeProvider } from '@/components/theme-provider';

const spaceGrotesk = Space_Grotesk({ 
  subsets: ['latin'],
  weight: ['300', '400', '500', '600', '700'],
  variable: '--font-space-grotesk',
});

export const metadata: Metadata = {
  title: 'LEAN Chain - Lightning-fast Enterprise Blockchain',
  description: 'Lightning-fast, enterprise-ready, adaptive blockchain network. Ultra-fast transactions, low costs, and full DeFi capabilities.',
  keywords: 'blockchain, cryptocurrency, DeFi, smart contracts, LEAN Chain',
  authors: [{ name: 'LEAN Chain' }],
  creator: 'LEAN Chain',
  publisher: 'LEAN Chain',
  icons: {
    icon: [
      { url: '/favicon.ico', sizes: 'any' },
      { url: '/lean_icon.png', type: 'image/png', sizes: '32x32' },
    ],
    shortcut: '/favicon.ico',
    apple: [
      { url: '/lean_icon.png', sizes: '180x180', type: 'image/png' },
    ],
  },
  openGraph: {
    type: 'website',
    locale: 'en_US',
    siteName: 'LEAN Chain',
    title: 'LEAN Chain - Lightning-fast Enterprise Blockchain',
    description: 'Lightning-fast, enterprise-ready, adaptive blockchain network. Ultra-fast transactions, low costs, and full DeFi capabilities.',
    images: [
      {
        url: '/lean_icon.png',
        width: 1200,
        height: 630,
        alt: 'LEAN Chain',
        type: 'image/png',
      },
    ],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'LEAN Chain - Lightning-fast Enterprise Blockchain',
    description: 'Lightning-fast, enterprise-ready, adaptive blockchain network. Ultra-fast transactions, low costs, and full DeFi capabilities.',
    images: ['/lean_icon.png'],
    creator: '@leanchain',
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  metadataBase: (() => {
    try {
      let baseUrl = 'https://leanchain.io';
      
      // Check NEXT_PUBLIC_BASE_URL - ensure it's not empty and is a valid URL
      if (process.env.NEXT_PUBLIC_BASE_URL && process.env.NEXT_PUBLIC_BASE_URL.trim()) {
        const url = process.env.NEXT_PUBLIC_BASE_URL.trim();
        // Validate it's a proper URL format
        if (url.startsWith('http://') || url.startsWith('https://')) {
          baseUrl = url;
        }
      } else if (process.env.VERCEL_URL && process.env.VERCEL_URL.trim()) {
        // Vercel URL should be just the hostname, add https://
        baseUrl = `https://${process.env.VERCEL_URL.trim()}`;
      }
      
      // Validate the URL before returning
      const url = new URL(baseUrl);
      return url;
    } catch (error) {
      // If anything fails, use the default
      console.warn('Failed to construct metadataBase URL, using default:', error);
      return new URL('https://leanchain.io');
    }
  })(),
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={spaceGrotesk.variable} suppressHydrationWarning>
      <body className={spaceGrotesk.className}>
        <ThemeProvider
          attribute="class"
          defaultTheme="dark"
          enableSystem={false}
          disableTransitionOnChange
        >
          {children}
        </ThemeProvider>
      </body>
    </html>
  );
}


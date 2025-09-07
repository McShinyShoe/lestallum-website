"use client";

import { useState, useEffect, useRef } from "react";
import Link from "next/link";
import { DarkModeToggle } from "./DarkModeToggle";

function NavbarList() {
  return (
    <>
      <Link href="/" className="block hover:bg-secondary">
        Home
      </Link>
      <Link href="/about" className="block hover:bg-secondary">
        About
      </Link>
      <Link href="/map" className="block hover:bg-secondary">
        Map
      </Link>
      <Link href="/rules" className="block hover:bg-secondary">
        Rules
      </Link>
      <Link href="/task" className="block hover:bg-secondary">
        Task
      </Link>
    </>
  );
}

export default function Navbar() {
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isClient, setIsClient] = useState(false); // To detect if we're on the client side
  const dropdownRef = useRef<HTMLDivElement | null>(null);
  const profileRef = useRef<HTMLDivElement | null>(null);

  const toggleDropdown = () => setIsDropdownOpen(!isDropdownOpen);
  const toggleMobileMenu = () => setIsMobileMenuOpen(!isMobileMenuOpen);

  useEffect(() => {
    setIsClient(true); // Set client flag once the component is mounted on the client side
  }, []);

  useEffect(() => {
    const handleDropdownPosition = () => {
      if (dropdownRef.current && profileRef.current) {
        const dropdown = dropdownRef.current;
        const profile = profileRef.current;

        const dropdownHeight = dropdown.offsetHeight;
        const profileRect = profile.getBoundingClientRect();
        const screenHeight = window.innerHeight;

        // If the dropdown goes out of the screen, reposition it
        if (profileRect.bottom + dropdownHeight > screenHeight) {
          dropdown.style.top = `-${dropdownHeight}px`; // Move it upwards
        } else {
          dropdown.style.top = `${profileRect.height + 8}px`; // Default position below the profile
        }
      }
    };

    if (isDropdownOpen) {
      handleDropdownPosition();
    }

    // Recalculate dropdown position on window resize
    window.addEventListener("resize", handleDropdownPosition);
    return () => {
      window.removeEventListener("resize", handleDropdownPosition);
    };
  }, [isDropdownOpen]);

  if (!isClient) {
    // Skip rendering on the server
    return null;
  }

  return (
    <nav className="fixed w-full bg-background text-foreground shadow-md h-[60px]">
      <div className="max-w-7xl mx-auto px-4 py-3 flex justify-between items-center">
        {/* Logo */}
        <Link href="/" className="text-xl font-bold">
          Lestallum
        </Link>

        {/* Mobile Hamburger Menu */}
<div className="flex lg:hidden">
  <button
    onClick={toggleMobileMenu}
    className="relative w-8 h-8 flex items-center justify-center focus:outline-none text-gray-800 dark:text-foreground"
    aria-label="Toggle Menu"
  >
    <span
      className={`absolute block h-0.5 w-6 bg-current transform transition duration-300 ease-in-out
        ${isMobileMenuOpen ? "rotate-45 translate-y-0" : "-translate-y-2"}`}
    />
    <span
      className={`absolute block h-0.5 w-6 bg-current transition-all duration-300 ease-in-out
        ${isMobileMenuOpen ? "opacity-0" : "opacity-100"}`}
    />
    <span
      className={`absolute block h-0.5 w-6 bg-current transform transition duration-300 ease-in-out
        ${isMobileMenuOpen ? "-rotate-45 translate-y-0" : "translate-y-2"}`}
    />
  </button>
</div>

        {/* Desktop Menu */}
        <div className="hidden lg:flex space-x-6">
          <NavbarList />
        </div>

        {/* User Profile */}
        <div className="relative flex items-center space-x-2" ref={profileRef}>
          {/* User's avatar and username */}
          <DarkModeToggle />
          <div
            className="flex items-center space-x-2 cursor-pointer"
            onClick={toggleDropdown}
          >
            <img
              src="https://www.gravatar.com/avatar/mcshinyshoe" // Replace with dynamic user avatar URL
              alt="User Avatar"
              className="w-8 h-8 rounded-full"
            />
            <span className="text-sm">Username</span>
          </div>

          {/* Dropdown Menu */}
          <div
            ref={dropdownRef}
            className={`absolute right-0 bg-card text-foreground rounded-lg shadow-lg w-48 py-2 z-10 transition-transform duration-200 ${
              isDropdownOpen ? "scale-100" : "scale-0"
            }`}
            style={{
              transformOrigin: "top right",
            }}
          >
            <Link href="/profile" className="block px-4 py-2 hover:bg-secondary">
              Edit Profile
            </Link>
            <Link
              href="/settings"
              className="block px-4 py-2 hover:bg-secondary"
            >
              Settings
            </Link>
            <button
              className="block w-full text-left px-4 py-2 hover:bg-secondary"
              onClick={() => alert("Logged out")} // Add logout functionality here
            >
              Logout
            </button>
          </div>
        </div>
      </div>

      {/* Mobile Menu (Hamburger) */}
      {isMobileMenuOpen && (
        <div className="lg:hidden bg-card text-foreground p-4 space-y-4">
          <NavbarList />
        </div>
      )}
    </nav>
  );
}

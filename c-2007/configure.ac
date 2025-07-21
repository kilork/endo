#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ(2.61)
AC_INIT(FULL-PACKAGE-NAME, VERSION, BUG-REPORT-ADDRESS)
AM_INIT_AUTOMAKE
AC_CONFIG_SRCDIR([decode.c])
AC_CONFIG_HEADER([config.h])

# Checks for programs.
AC_PROG_CC

# Checks for libraries.

# Checks for header files.
AC_HEADER_STDC
AC_CHECK_HEADERS([stdlib.h string.h])

# Checks for typedefs, structures, and compiler characteristics.
AC_C_INLINE

# Checks for library functions.
AC_FUNC_MALLOC

PKG_CHECK_MODULES(GTK_PACKAGE, gtk+-2.0)
AC_SUBST(GTK_PACKAGE_CFLAGS)
AC_SUBST(GTK_PACKAGE_LIBS)

AC_CONFIG_FILES([Makefile])
AC_OUTPUT

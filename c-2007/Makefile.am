## Process this file with automake to produce Makefile.in

## Created by Anjuta

INCLUDES = \
	-DPACKAGE_LOCALE_DIR=\""$(prefix)/$(DATADIRNAME)/locale"\" \
	-DPACKAGE_SRC_DIR=\""$(srcdir)"\" \
	-DPACKAGE_DATA_DIR=\""$(datadir)"\" \
	$(GTK_PACKAGE_CFLAGS)

AM_CFLAGS = \
	 -Wall \
	 -g

bin_PROGRAMS = endo \
	rnaview

rnaview_SOURCES = \
	rnaview.c

rnaview_LDADD = \
	$(GTK_PACKAGE_LIBS)

endo_SOURCES = \
	main.c\
	decode.c

endo_LDFLAGS = 

endo_LDADD = \
	$(GTK_PACKAGE_LIBS)

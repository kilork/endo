#include <sys/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gtk/gtk.h>
#include <gdk/gdk.h>
#include <glib.h>

#define IMAGE_SIZE 600

struct {
	int r;
	int g;
	int b;
	int c;
	int a;
	int ca;
} bucket;

typedef struct _color {
	int r;
	int g;
	int b;
	int a;
} color;

typedef struct _bitmap bitmap;
struct _bitmap {
	color* grid;
	bitmap* next;
};

int posx = 0, posy = 0;
int markx = 0, marky = 0;
int dir = 1;
int dirsx[] =  {0, 1, 0, -1};
int dirsy[] =  {-1, 0, 1, 0};
color* currentpx;
color old;
FILE* f;
char cmd[7];
bitmap* bitmaps = NULL;
int bitmaps_count = 0;

void bitmaps_add(bitmap* b) {
	printf("bitmaps_add()\n");
	if(bitmaps_count == 10) return;
	b->next = bitmaps;
	bitmaps = b;
	bitmaps_count++;
}

void bitmaps_remove() {
	bitmap* b = bitmaps;
	bitmaps = b->next;
	free(b->grid);
	free(b);
	bitmaps_count--;
}

bitmap* bitmap_new(int a) {
	bitmap* b = g_new(bitmap, 1);
	int i;
	color* c;
	b->grid = g_new(color,IMAGE_SIZE*IMAGE_SIZE);
	for(i = 0; i < IMAGE_SIZE*IMAGE_SIZE; i++) {
		c = b->grid + i;
		c->r = c->g = c->b = 0;
		c->a = a;
	}
	return b;
}


void flash_bucket() {
	printf("flash_bucket\n");
	bucket.r = 0;
	bucket.g = 0;
	bucket.b = 0;
	bucket.c = 0;
	bucket.a = 0;
	bucket.ca = 0;
	currentpx->r = currentpx->g = currentpx->b = 0;
	currentpx->a = 255;
}

void currentPixel() {
	if(bucket.c + bucket.ca == 0) return;
	currentpx->a = bucket.ca ? bucket.a / bucket.ca : 255;
	currentpx->r = bucket.c ? bucket.r * currentpx->a / 255 / bucket.c : 0;
	currentpx->g = bucket.c ? bucket.g * currentpx->a / 255 / bucket.c : 0;
	currentpx->b = bucket.c ? bucket.b * currentpx->a / 255 / bucket.c : 0;
	printf("currentPixel(%d, %d, %d, %d)\n", currentpx->r, currentpx->g, currentpx->b, currentpx->a);
}

int inline max(int a, int b) { return a > b ? a : b; }

void setPixel(int x, int y) {
	color *c = bitmaps->grid + y * IMAGE_SIZE + x;
	c->a = currentpx->a;
	c->r = currentpx->r;
	c->g = currentpx->g;
	c->b = currentpx->b;
}

color* getPixel(int x, int y) {
	return bitmaps->grid + y * IMAGE_SIZE + x;
}

void line() {
	int deltax = markx - posx;
	int deltay = marky - posy;
	int d = max(abs(deltax), abs(deltay));
	int c = deltax * deltay > 0 ? 0 : 1;
	int x, y, i;
	currentPixel();
	printf("line(%d, %d, %d, %d)\n", posx, posy, markx, marky);
	for(i = 0, x = posx * d + ((d - c) >> 1), y = posy * d + ((d - c) >> 1); i < d; i++, x += deltax, y += deltay) {
		setPixel(x / d, y / d);
	}
	setPixel(markx, marky);
}

int cmpcolor(color* a) {
  //printf("cmp a %d %d %d %d\n", a->r, a->g, a->b, a->a);
  //printf("cmp old %d %d %d %d\n", old.r, old.g, old.b, old.a);
  return  a->a == old.a && a->r == old.r && a->g == old.g && a->b == old.b;
}

int lineFill(int x, int y, int dir, int prevxl, int prevxr) {
  int xl = x;
  int xr = x;
  color* c;
  do c = getPixel(--xl, y); while(xl > 0 && cmpcolor(c));
  do c = getPixel(++xr, y); while(xr < IMAGE_SIZE - 1 && cmpcolor(c));
  xl++; xr--;
  int i;
  printf("fill %d-%d %d\n", xl, xr, y);
  for(i = xl; i <= xr; i++)
    setPixel(i, y);
  if(y + dir != -1 && y + dir != IMAGE_SIZE) 
    for(x = xl; x <= xr; x++) {
      c = getPixel(x, y + dir);
      if(cmpcolor(c))
	x = lineFill(x, y + dir, dir, xl, xr);
    }
  if(y - dir != -1 && y - dir != IMAGE_SIZE) {
    for(x = xl; x < prevxl; x++) {
      c = getPixel(x, y - dir);
      if(cmpcolor(c))
	x = lineFill(x, y - dir, -dir, xl, xr);
    }
    for(x = prevxr; x < xr; x++) {
      c = getPixel(x, y - dir);
      if(cmpcolor(c))
	x = lineFill(x, y - dir, -dir, xl, xr);
    }
  }
  return xr;
}

void tryFill() {
	old = *getPixel(posx, posy);
	currentPixel();
	printf("fill(%d, %d, o:(%d,%d,%d,%d),n:(%d,%d,%d,%d))\n", posx, posy, old.r, old.g, old.b, old.a, currentpx->r, currentpx->g, currentpx->b, currentpx->a
);
	if(!cmpcolor(currentpx))
	  lineFill(posx, posy, 1, posx, posx);
	printf("end fill\n");
}

void compose() {
	int i;
	color *c0, *c1;
	int d;
	printf("compose()\n");
	if(bitmaps_count < 2) return;
	for(i = 0, c0 = bitmaps->grid, c1 = bitmaps->next->grid; i < IMAGE_SIZE * IMAGE_SIZE; i++, c0++, c1++) {
		d = 255 - c0->a;
		c1->r = c0->r + (int)(c1->r * d / 255);
		c1->g = c0->g + (int)(c1->g * d / 255);
		c1->b = c0->b + (int)(c1->b * d / 255);
		c1->a = c0->a + (int)(c1->a * d / 255);
	}
	bitmaps_remove();
}

void clip() {
	int i;
	color *c0, *c1;
	printf("clip()\n");
	if(bitmaps_count < 2) return;
	for(i = 0, c0 = bitmaps->grid, c1 = bitmaps->next->grid; i < IMAGE_SIZE * IMAGE_SIZE; i++, c0++, c1++) {
		c1->r = c1->r * c0->a / 255;
		c1->g = c1->g * c0->a / 255;
		c1->b = c1->b * c0->a / 255;
		c1->a = c1->a * c0->a / 255;
	}
	bitmaps_remove();
}

void draw() {
	int i;
	bitmap* b = bitmap_new(0);
	flash_bucket();
	bitmaps_add(b);
	while((*cmd = fgetc(f)) != EOF) {
		for(i = 1; i < 7; i++)
			cmd[i] = fgetc(f);
		if(strncmp("PIPIIIC", cmd, 7) == 0) bucket.c++; // black
		else if(strncmp("PIPIIIP", cmd, 7) == 0) bucket.c++, bucket.r += 255; // red
		else if(strncmp("PIPIICC", cmd, 7) == 0) bucket.c++, bucket.g += 255; // green
		else if(strncmp("PIPIICF", cmd, 7) == 0) bucket.c++, bucket.r += 255, bucket.g += 255; // yellow
		else if(strncmp("PIPIICP", cmd, 7) == 0) bucket.c++, bucket.b += 255; // blue
		else if(strncmp("PIPIIFC", cmd, 7) == 0) bucket.c++, bucket.r += 255, bucket.b += 255; // magenta
		else if(strncmp("PIPIIFF", cmd, 7) == 0) bucket.c++, bucket.g += 255, bucket.b += 255; // cyan
		else if(strncmp("PIPIIPC", cmd, 7) == 0) bucket.c++, bucket.r += 255, bucket.g += 255, bucket.b += 255; // white
		else if(strncmp("PIPIIPF", cmd, 7) == 0) bucket.ca++; // transparent
		else if(strncmp("PIPIIPP", cmd, 7) == 0) bucket.ca++, bucket.a += 255; // opaque
		else if(strncmp("PIIPICP", cmd, 7) == 0) flash_bucket(); // e
		else if(strncmp("PIIIIIP", cmd, 7) == 0) posx = (IMAGE_SIZE + posx + dirsx[dir]) % IMAGE_SIZE, posy = (IMAGE_SIZE + posy + dirsy[dir]) % IMAGE_SIZE; // move
		else if(strncmp("PCCCCCP", cmd, 7) == 0) dir = (4 + dir - 1) % 4; // turnCounterClockwise
		else if(strncmp("PFFFFFP", cmd, 7) == 0) dir = (4 + dir + 1) % 4; // turnClockwise
		else if(strncmp("PCCIFFP", cmd, 7) == 0) markx = posx, marky = posy; // mark <- pos
		else if(strncmp("PFFICCP", cmd, 7) == 0) line();
		else if(strncmp("PIIPIIP", cmd, 7) == 0) tryFill();
		else if(strncmp("PCCPFFP", cmd, 7) == 0) b = bitmap_new(0), bitmaps_add(b);
		else if(strncmp("PFFPCCP", cmd, 7) == 0) compose();
		else if(strncmp("PFFICCF", cmd, 7) == 0) clip();
		else printf("%c%c%c%c%c%c%c\n", cmd[0], cmd[1], cmd[2], cmd[3], cmd[4], cmd[5], cmd[6]);
	}
}

int main(int argc, char**argv) {
	f = fopen(argv[1], "r");
	currentpx = malloc(sizeof(color));
	draw();
	gtk_set_locale();
	gtk_init(&argc, &argv);
	// create main window
	GtkWidget* windowMain = gtk_window_new(GTK_WINDOW_TOPLEVEL);
	gtk_window_resize((GtkWindow*)windowMain, IMAGE_SIZE, IMAGE_SIZE);
	g_signal_connect(windowMain, "destroy", G_CALLBACK(gtk_main_quit), NULL);
	
	
	// create pixmap
	GdkPixmap* gdkpixmap = gdk_pixmap_new(NULL, IMAGE_SIZE, IMAGE_SIZE, 24);
	if(!gdkpixmap) { printf("couldn't init gdkpixmap!\n"); exit(0); }
	GdkGC* gc = gdk_gc_new(gdkpixmap);
	GdkColormap* cmap = gdk_gc_get_colormap(gc);
	GdkColor c;
	gdk_color_parse("#000000", &c);
	gdk_colormap_alloc_color(cmap, &c, FALSE, TRUE);
	gdk_gc_set_foreground(gc, &c);
	gdk_draw_rectangle(gdkpixmap, gc, TRUE, 0, 0, IMAGE_SIZE, IMAGE_SIZE);

	color* pc;
	int i;
	for(i = 0, pc = bitmaps->grid; i < IMAGE_SIZE * IMAGE_SIZE; i++, pc++) {
		c.pixel = 0;
		c.red = pc->r * 257;
		c.green = pc->g * 257;
		c.blue = pc->b * 257;
		gdk_colormap_alloc_color(cmap, &c, FALSE, TRUE);
		gdk_gc_set_foreground(gc, &c);
		gdk_draw_point(gdkpixmap, gc, i % IMAGE_SIZE, i / IMAGE_SIZE);
	}
	
	// free bitmaps
	printf("free bitmaps..");
	while(bitmaps_count) { printf("%d\n", bitmaps_count); bitmaps_remove(); };
	// create image
	GtkImage* image = (GtkImage*)gtk_image_new_from_pixmap(gdkpixmap, NULL);
	gtk_container_add(GTK_CONTAINER(windowMain), (GtkWidget*)image);

	gtk_widget_show_all(windowMain);
	gtk_main();
	g_object_unref(gc);
	g_object_unref(gdkpixmap);
	printf("quit\n");
	return 0;
}

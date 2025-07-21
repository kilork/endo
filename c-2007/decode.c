/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Library General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor Boston, MA 02110-1301,  USA
 */
 
#include <stdio.h>
#include <stdlib.h>
#include <glib.h>

#define INDEX_SIZE 5000
#define FULL_INDEX_SIZE 30000000
typedef struct _list list;
struct _list {
  int c;
  int v;
  list* n;
  list* consts;
};

list rna = {0,0,0,0};
list dna = {0,0,0,0};
list* pdna = &dna;
list* prna = &rna;
list* buff = 0;
int dnalen = 0, dnalenold;

list** index;

list* index_search(int i) {
  int j, k, n;
  j = i / INDEX_SIZE;
  if(j >= FULL_INDEX_SIZE / INDEX_SIZE) { printf("!!!!!!\n"); return NULL; }
  list* item = index[j];
  if(!item) {
    while(j >= 0 && !index[j]) j--;
    if(j == -1) {
      // empty index, need to fill first one
      j = 0;
      index[j] = pdna;
    }
    item = index[j];
  }
  k = i - j * INDEX_SIZE;
  n = 0;
  while(n != k && item) {
    item = item->n;
    n++;
    if(n % INDEX_SIZE == 0) {
      j++;
      index[j] = item;
    }
  }
  return item;
}

void rebuild_index() {
  int diff = dnalen - dnalenold, shift, i;
  shift = diff / INDEX_SIZE;
  if(diff < 0) {
    if(shift) {
      for(i = 0; i - shift < FULL_INDEX_SIZE / INDEX_SIZE; i++)
	index[i] = index[i - shift];
      for(; i < FULL_INDEX_SIZE / INDEX_SIZE; i++)
	index[i] = NULL;
    }
    diff = diff % INDEX_SIZE;
    while(diff++)
      for(i = 0; i < FULL_INDEX_SIZE / INDEX_SIZE; i++)
	if(index[i])
	  index[i] = index[i]->n;
  }
  else if(diff > 0) {
    shift++;
    for(i = FULL_INDEX_SIZE / INDEX_SIZE - 1; i - shift >= 0; i--)
      index[i] = index[i - shift];
    for(;i >= 0;i--)
      index[i] = NULL;
    diff = INDEX_SIZE - (diff % INDEX_SIZE);
    while(diff--)
      for(i = 0; i < FULL_INDEX_SIZE / INDEX_SIZE; i++)
	if(index[i])
	  index[i] = index[i]->n;
  }
  dnalenold = dnalen;
}

//void flush_index() {
//  int c = (dnalenold - dnalen) / INDEX_SIZE;
//  for(int i = 0; i < 
//}

FILE *outf, *debugf;

void finish();

char readchar() {
  char c = EOF;
  list* p;
  if(buff != 0) { 
    c = buff->c;
    p = buff;
    buff = buff->n;
  }
  else if(pdna) {
    c = pdna->c;
    p = pdna;
    pdna = pdna->n;
  }
  else finish();
  g_slice_free(list, p);
  dnalen--;
  fputc(c, debugf);
  return c;
}

void printlistn(list* p, int n) {
	while(p && --n) {
	  printf("%c", p->c);
	  p = p->n; 
	}
}
void printlist20(list* p) {
  printlistn(p, 80);
}

void printlist(list* p) {
  while(p) {
    switch(p->c) {
    case 'I':
    case 'C':
    case 'F':
    case 'P':
    case '(':
    case ')':
      printf("%c", p->c);
      break;
    case '!':
      printf("![%d]", p->v);
      break;
    case '?':
      printf("?["); printlist(p->consts); printf("]");
      break;
    default:
      if(p->consts)
	printf("|n:%d| ", p->v);
      else
	printf("Nl(%d,%d)", p->v, -p->c);
    }
    p = p->n; 
  }
}

void finish() {
  fputc(EOF, outf);
  fclose(outf);
  fputc(EOF, debugf);
  fclose(debugf);
  printf("finish\n");
  exit(0);
}

int cmp(list* a, list* b) {
  int count = 0;
  while(a && b) {
    if(a->c - b->c) return 0;
    count++;
    a = a->n; b = b->n;
  }
  if(b) return 0;
  return count;
}

list* addf(list* l, int c, int v, list* consts) {
  list* _ = g_slice_new(list);
  _->c = c;
  _->v = v;
  _->consts = consts;
  _->n = 0;
  l->n = _;
  return _;
}

list* addv(list* l, int c, int v) {
  return addf(l, c, v, 0);
}

list* addc(list* l, int c, list* consts) {
  return addf(l, c, 0, consts);
}

list* add(list* l, char c) {
  return addf(l, c, 0, 0);
}

int nat(char c) {
  int res = 0, level = 0, max = 0;
  while(c != 'P') {
    if(c == 'C') {
      if(level > 30) return 1 << 30;
      res += 1 << level;
    }
    c = readchar();
    level++;
    if(c == EOF) finish();
  }
  return res;
}

list* consts(char c) {
  list* l = g_slice_new(list);
  char c2;
  switch(c) {
  case 'I': switch(c2 = readchar()) {
    case 'C': l->c = 'P'; break;
    default: l->c = c2; l->n = buff; buff = l; l = g_slice_new(list); l->c = c; l->n = buff; buff = l; dnalen += 2; return 0;
    }
    break;
  case 'C': l->c = 'I'; break;
  case 'F': l->c = 'C'; break;
  case 'P': l->c = 'F'; break;
  }
  l->n = consts(readchar());
  return l;
}

void pattern(list* p_head) {
  list* p = p_head;
  int lvl = 0, i;
  while(1) {
    switch(readchar()) {
    case 'C': p = add(p, 'I'); break;
    case 'F': p = add(p, 'C'); break;
    case 'P': p = add(p, 'F'); break;
    case 'I': switch(readchar()) {
      case 'C': p = add(p, 'P'); break;
      case 'P': p = addv(p, '!', nat(readchar())); break;
      case 'F': readchar(); p = addc(p, '?', consts(readchar())); break;
      case 'I': switch(readchar()) {
	case 'P': lvl++; p = add(p, '('); break;
	case 'C':
	case 'F': if(lvl--) p = add(p, ')'); else return; break;
	case 'I': for(i = 0; i < 7; i++) prna = add(prna, readchar()), fputc(prna->c, outf); break;
	}
	break;
      } 
      break;
    default: finish();
    }
    fputc('\n', debugf);
  }
}

void template(list* p_head) {
  list* p = p_head;
  int l, n;
  while(1) {
    switch(readchar()) {
    case 'C': p = add(p, 'I'); break;
    case 'F': p = add(p, 'C'); break;
    case 'P': p = add(p, 'F'); break;
    case 'I': switch(readchar()) {
      case 'C': p = add(p, 'P'); break;
      case 'F':
      case 'P': l = nat(readchar()); n = nat(readchar()); p = addf(p, -l, n, 0); break;
      case 'I': switch(readchar()) {
	case 'C':
	case 'F':
	  return;
	case 'P': p = addf(p, 0, nat(readchar()), (list*)1); break;
	case 'I': for(n = 0; n < 7; n++) prna = add(prna, readchar()), fputc(prna->c, outf); break;
	}
	break;
      }
      break;
    }
    fputc('\n', debugf);
  }
}

void freelist(list* p) {
  list* tmp;
  while(tmp = p) {
    p = p->n; 
    if(tmp->c == '?' && tmp->consts) 
      freelist(tmp->consts);
    g_slice_free(list, tmp); 
  } 
}

list* asnat(list* l, int len) {
  //printf("asnat(%d)\n", len);
  while(len) {
    l = add(l, len % 2 ? 'C' : 'I');
    len /= 2;
  }
  l = add(l, 'P');
  return l;
}


list* protect(list* s, int l, list* d, int len) {
  list* p = s, *tmp;
  int cnt = len;
  while(cnt && d) { 
    s = add(s, d->c); 
    d = d->n;
    cnt--;
  }
  while(l--) {
    d = tmp = p;
    while(d = d->n) {
      switch(d->c) {
      case 'I': d->c = 'C'; break;
      case 'C': d->c = 'F'; break;
      case 'F': d->c = 'P'; break;
      case 'P': d->c = 'C'; tmp->n = g_slice_new(list); tmp->n->c = 'I'; tmp->n->n = d; break;
      }
      tmp = d;
    }
  }
  return s;
}

void replace(list* t, list* e) {
  //printf("replace()\ne["); printlist20(e); printf("...]\n");
  list r = {0,0,0,0};
  list* pr = &r;
  list* tmp, *printl;
  int len;
  while(t) {
    if(t->c > 0) {
      pr = add(pr, t->c);
    }
    else if(t->consts) {
      tmp = e;
      len = t->v;
      while(len-- && tmp) tmp = tmp->n;
      len = 0;
      if(tmp) {
	len = tmp->v;
      }
      //printl = pr;
      pr = asnat(pr, len);
      //printf("asnat:[");printlist20(printl->n);printf("...]\n");
    }
    else {
      tmp = e;
      len = t->v;
      while(len-- && tmp) tmp = tmp->n;
      //printl = pr;
      if(tmp)
	pr = protect(pr, -t->c, tmp->consts, tmp->v);
      //printf("protect:[");printlist20(printl->n);printf("...]\n");
    }
    t = t->n;
  }
  len = 0;
  tmp = r.n;
  while(tmp) { tmp = tmp->n; len++; }
  dnalen += len;
  pr->n = pdna;
  pdna = r.n;
  //printf("end replace();\n");
}

void matchreplace(list* p, list* t) {
  int i = 0, len, count, fast_index = 0;
  list *tmp, *d, *fast = pdna;
  list* c = 0;
  list e = {0,0,0,0};
  list* pe = &e;
  rebuild_index();
  while(p) {
    switch(p->c) {
    case '!': 
      i = i + p->v;
      if(i > dnalen) goto matchreplace_finally;
      break;
    case '(':
      tmp = g_slice_new(list);
      tmp->c = i;
      tmp->n = c;
      c = tmp;
      break;
    case ')':
      //printf("e+=dna[%d,%d]\n", c->c, i);
      len = i - c->c;
      if(len < 0) len = 0;
      //if(!len) printf("zero len!\n");
      c->v = c->c;
      // optimize search
      count = c->c - fast_index;
      if(count >= 0 && count < INDEX_SIZE / 2) {
	d = fast;
	c->c -= fast_index;
	while(c->c--) d = d->n;
      }
      else
	d = index_search(c->c);
      pe = add(pe, '-');
      pe->consts = d;
      if(c->v > fast_index) {
	fast_index = c->v;
	fast = d;
      }
      if(c->v + len > dnalen) {
	//printf("overflow!\n");
	len = dnalen - c->v;
      }
      pe->v = len;
      tmp = c;
      c = c->n;
      g_slice_free(list, tmp);
      break;
    case '?':
      len = i;
      count = i - fast_index;
      if(count >= 0 && count < INDEX_SIZE / 2) {
	d = fast;
	len -= fast_index;
	while(len--) d = d->n;
      }
      else
	d = index_search(i);
      if(i > fast_index) {
	fast_index = i;
	fast = d;
      }
      
      
      len = 0;
      while(d) {
	if((count = cmp(d, p->consts)) != 0) break;
	len++;
	d = d->n;
      }
      if(!d) goto matchreplace_finally;
      i = i + len + count;
      //printf("found "); printlist(p->consts); printf(" at %d\n", i);
      break;
    default:
      len = i;
      count = i - fast_index;
      if(count >= 0 && count < INDEX_SIZE / 2) {
	d = fast;
	len -= fast_index;
	while(len--) d = d->n;
      }
      else
	d = index_search(i);
      if(i > fast_index) {
	fast_index = i;
	fast = d;
      }
      
      if(d->c != p->c) goto matchreplace_finally;
      i++;
    }
    p = p->n;
  }
  dnalen -= i;
  tmp = pdna;
  count = i - fast_index;
  if(count >= 0 && count < INDEX_SIZE / 2) {
    pdna = fast;
    i -= fast_index;
    while(i--) pdna = pdna->n;
  }
  else
    pdna = index_search(i);
  rebuild_index();
  p = pdna;
  replace(t, e.n);
  rebuild_index();
  while(tmp != p) { d = tmp; tmp = tmp->n; g_slice_free(list, d); }
  //printf("dnalen: %d\n", dnalen);
  
 matchreplace_finally:
  freelist(e.n);
  freelist(c);
}

void parse() {
  int i = 0;
  while(1) {
    dnalenold = dnalen;
    list* p = g_slice_new(list);
    p->n = 0;
    list* t = g_slice_new(list);
    t->n = 0;
    //printf("dna["); printlist20(pdna); printf("...]\npattern:\n"); 
    pattern(p);
    //printlist(p->n);
    //printf("\ndna["); printlist20(pdna); printf("...]\ntemplate:\n"); 
    template(t);
    //printlist(t->n);
    //printf("\ndna["); printlist20(pdna); printf("...]\nmatchreplace:\n"); 
    matchreplace(p->n, t->n);
    printf("\ndnalen:%d i:%d\ndna[", dnalen, i++);
    if(dnalen > 100) {
      printlist20(pdna); printf("...]\n\n"); 
    }
    else {
      printlist(pdna); printf("]\n\n"); 
    }
    freelist(p);
    freelist(t);
  }
}

void proceed(char* prefix, char* input, char* output, char* debug)
{
  char c;
  index = g_new0(list*, FULL_INDEX_SIZE / INDEX_SIZE);
  while(c = *prefix) { 
    pdna = add(pdna, c);
    if(dnalen % INDEX_SIZE == 0) index[dnalen / INDEX_SIZE] = pdna;
    dnalen++;
    prefix++; 
  }
  FILE* f = fopen(input, "r");
  while((c = fgetc(f)) != EOF) { 
    pdna = add(pdna, c); 
    if(dnalen % INDEX_SIZE == 0) index[dnalen / INDEX_SIZE] = pdna;
    dnalen++; 
  }
  fclose(f);
  pdna = dna.n;
  outf = fopen(output, "w");
  debugf = fopen(debug, "w");
  parse();
}

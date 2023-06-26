#include "geosteiner.h"
#include "stdlib.h"

struct C_ESMT {
double length;
int nsps;
double * sps;
int nedges;
int * edges;
};

struct C_ESMT rs_compute_esmt (int nterms, double * terms)
    {
    int i, nsps, nedges, edges [10000];
    double length, sps [1000];

    /* Open GeoSteiner environment */
    if (gst_open_geosteiner () != 0) {
        printf ("Could not open GeoSteiner.\n");
        exit (1);
    }

    /* Compute Euclidean Steiner tree */
    gst_esmt (nterms, terms, &length, &nsps, sps, &nedges, edges, NULL, NULL);

    for (i = 0; i < nsps; i++) {
        printf ("Steiner point: (%f, %f)\n", sps[2*i], sps[2*i+1]);
    }

    for (i = 0; i < nedges; i++) {
        printf ("Edge: (%i, %i)\n", edges[2*i], edges[2*i+1]);
    }

    /* Close GeoSteiner environment */
    gst_close_geosteiner ();

    struct C_ESMT c_esmt;
    c_esmt.length = length;
    c_esmt.nsps = nsps;
    c_esmt.sps = sps;
    c_esmt.nedges = nedges;
    c_esmt.edges = edges;

	return c_esmt;
}

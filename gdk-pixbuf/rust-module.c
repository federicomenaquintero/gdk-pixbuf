/* -*- Mode: C; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 8 -*- */
/* vim: set sw=8 sts=8 expandtab: */
/* GdkPixbuf library - C stubs for pixbuf modules in Rust
 *
 * Copyright (C) 2017 Federico Mena Quintero
 *
 * Authors: Federico Mena Quintero <federico@gnome.org>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#include "config.h"
#include "gdk-pixbuf-private.h"

typedef struct {
        GdkPixbufModuleSizeFunc     size_func;
        GdkPixbufModulePreparedFunc prepared_func;
        GdkPixbufModuleUpdatedFunc  updated_func;
        gpointer                    user_data;
} LoadContext;

G_GNUC_INTERNAL
gpointer rust_loader_new (LoadContext *load_context);

G_GNUC_INTERNAL
void load_context_notify_size (LoadContext *load_context, gint *width, gint *height);

G_GNUC_INTERNAL
void load_context_notify_prepared (LoadContext *load_context, GdkPixbuf *pixbuf, GdkPixbufAnimation *animation);

static LoadContext *
load_context_new (GdkPixbufModuleSizeFunc     size_func,
                  GdkPixbufModulePreparedFunc prepared_func,
                  GdkPixbufModuleUpdatedFunc  updated_func,
                  gpointer                    user_data,
                  GError                    **error)
{
        LoadContext *load_context = g_new0 (LoadContext, 1);
        GError *my_error = NULL;

        g_assert (size_func != NULL);
        g_assert (prepared_func != NULL);
        g_assert (updated_func != NULL);

        load_context->size_func     = size_func;
        load_context->prepared_func = prepared_func;
        load_context->updated_func  = updated_func;
        load_context->user_data     = user_data;

        load_context->rust_loader = rust_loader_new (load_context);

        g_propagate_error (error, my_error);

        return load_context;        
}

static gboolean
load_context_stop_load (LoadContext *load_context,
                        GError     **error)
{
        GError *my_error = NULL;
        gboolean retval;

        retval = /* FIXME: call the Rust context */;

        g_propagate_error (error, my_error);
        return retval;
}

static gboolean
load_context_load_increment (LoadContextg  *load_context,
                             const guchar  *buf,
                             guint          size,
                             GError       **error)
{
        GError *my_error = NULL;

        retval = /* FIXME: call the Rust context */;

        g_propagate_error (error, my_error);
        return retval;
}

/*** Functions callable from Rust ***/

void
load_context_notify_size (LoadContext *load_context,
                          gint        *width,
                          gint        *height)
{
        load_context->size_func (width, height, load_context->user_data);
}

void
load_context_notify_prepared (LoadContext        *load_context,
                              GdkPixbuf          *pixbuf,
                              GdkPixbufAnimation *animation)
{
        load_context->prepared_func (pixbuf, animation, load_context->user_data);
}

void
load_context_notify_updated (LoadContext *load_context,
                             GdkPixbuf   *pixbuf,
                             int          x,
                             int          y,
                             int          width,
                             int          height)
{
        load_context->updated_func (pixbuf, x, y, width, height, load_context->user_data);
}

/*** GdkPixbufModule vtable implementations ***/

static gpointer
begin_load (GdkPixbufModuleSizeFunc     size_func,
            GdkPixbufModulePreparedFunc prepared_func,
            GdkPixbufModuleUpdatedFunc  updated_func,
            gpointer                    user_data,
            GError                    **error)
{
        return load_context_new (size_func, prepared_func, updated_func, user_data, error);
}

static gboolean
stop_load (gpointer   data,
           GError   **error)
{
        LoadContext *load_context = data;
        gboolean retval;

        retval = load_context_stop_load (load_context, error);

        load_context_free (load_context);

        return retval;
}

static gboolean
load_increment (gpointer       data,
                const guchar  *buf,
                guint          size,
                GError       **error)
{
        LoadContext *load_context = data;

        return load_context_load_increment (load_context, buf, size, error);
}

/*** GModule entry points ***/

G_MODULE_EXPORT void
fill_vtable (GdkPixbufModule *module)
{
        module->begin_load     = begin_load;
        module->stop_load      = stop_load;
        module->load_increment = load_increment;
}

G_MODULE_EXPORT void
fill_info (GdkPixbufFormat *info)
{
        static const GdkPixbufModulePattern signature[] = {
                { "GIF8", NULL, 100 },
                { NULL, NULL, 0 }
        };
        static const gchar *mime_types[] = {
                "image/gif",
                NULL
        };
        static const gchar *extensions[] = {
                "gif",
                NULL
        };

        info->name = "gif";
        info->signature = (GdkPixbufModulePattern *) signature;
        info->description = NC_("image format", "GIF");
        info->mime_types = (gchar **) mime_types;
        info->extensions = (gchar **) extensions;
        info->flags = GDK_PIXBUF_FORMAT_THREADSAFE;
        info->license = "LGPL";
}

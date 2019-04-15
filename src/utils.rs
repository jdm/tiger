use euclid::*;

use crate::sheet::Animation;
use crate::streamer::TextureCache;

pub struct Fill {
    pub rect: Rect<f32>,
    pub zoom: f32,
}

pub fn fill(space: Size2D<f32>, content_size: Size2D<f32>) -> Option<Fill> {
    if content_size.is_empty_or_negative() {
        return None;
    }
    if space.is_empty_or_negative() {
        return None;
    }

    let aspect_ratio = content_size.width / content_size.height;
    let fit_horizontally =
        (content_size.width / space.width) >= (content_size.height / space.height);

    let (w, h);
    if fit_horizontally {
        if space.width > content_size.width {
            w = content_size.width * (space.width / content_size.width).floor();
        } else {
            w = space.width;
        }
        h = w / aspect_ratio;
    } else {
        if space.height > content_size.height {
            h = content_size.height * (space.height / content_size.height).floor();
        } else {
            h = space.height;
        }
        w = h * aspect_ratio;
    }

    Some(Fill {
        rect: rect((space.width - w) / 2.0, (space.height - h) / 2.0, w, h),
        zoom: w / content_size.width,
    })
}

#[derive(Fail, Debug)]
pub enum BoundingBoxError {
    #[fail(display = "Animation is empty")]
    EmptyAnimation,
    #[fail(display = "Frame data not loaded")]
    FrameDataNotLoaded,
}

#[derive(Debug)]
pub struct BoundingBox {
    pub rect: Rect<i32>,
}

impl BoundingBox {
    pub fn center_on_origin(&mut self) {
        self.rect = Rect::<i32>::from_points(&[
            self.rect.origin,
            self.rect.origin * -1,
            self.rect.bottom_right(),
            self.rect.bottom_right() * -1,
        ]);
        let delta_origin = self.rect.origin.clone() * -1;
        let delta_size = self.rect.size.clone() / -2;
        self.rect = self.rect.translate(&delta_origin.to_vector());
        self.rect = self.rect.translate(&delta_size.to_vector());
    }
}

pub fn get_bounding_box(
    animation: &Animation,
    texture_cache: &TextureCache,
) -> Result<BoundingBox, BoundingBoxError> {
    if animation.get_num_frames() == 0 {
        return Err(BoundingBoxError::EmptyAnimation);
    }
    let mut bbox_rectangle = Rect::<i32>::zero();
    for frame in animation.frames_iter() {
        let texture = texture_cache
            .get(frame.get_frame())
            .ok_or(BoundingBoxError::FrameDataNotLoaded)?;
        let frame_offset = Vector2D::<i32>::from(frame.get_offset());
        let frame_rectangle =
            Rect::<i32>::from_size(texture.size.to_i32()).translate(&frame_offset);
        bbox_rectangle = bbox_rectangle.union(&frame_rectangle);
    }
    Ok(BoundingBox {
        rect: bbox_rectangle,
    })
}

#[test]
fn test_center_on_origin() {
    {
        let mut b = BoundingBox {
            rect: Rect::<i32>::new(point2(-50, -300), size2(1000, 800)),
        };
        b.center_on_origin();
        assert_eq!(
            b.rect,
            Rect::<i32>::new(point2(-950, -500), size2(1900, 1000))
        );
    }
    {
        let mut b = BoundingBox {
            rect: Rect::<i32>::new(point2(100, 100), size2(50, 50)),
        };
        b.center_on_origin();
        assert_eq!(
            b.rect,
            Rect::<i32>::new(point2(-150, -150), size2(300, 300))
        );
    }
}
